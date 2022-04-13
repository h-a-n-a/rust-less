use crate::extend::vec_str::VecCharExtend;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::NodeWeakRef;
use crate::new_less::scan::traversal;
use crate::new_less::token::lib::{Token, TokenInterface};
use crate::new_less::token::new_select::{
  TokenAllowChar, TokenCombinaChar, TokenKeyWordChar, TokenSelectChar,
};
use crate::new_less::var::HandleResult;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum SelectParadigm {
  SelectWrap(String),
  CominaWrap(TokenCombinaChar),
  VarWrap(String),
}

pub trait Paradigm {
  fn join(&self) -> String;
}

impl Paradigm for Vec<SelectParadigm> {
  fn join(&self) -> String {
    let mut txt = "".to_string();
    self.iter().for_each(|par| match par {
      SelectParadigm::SelectWrap(cc) | SelectParadigm::VarWrap(cc) => txt += cc,
      SelectParadigm::CominaWrap(cc) => {
        txt.push(cc.to_str());
      }
    });
    txt
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct NewSelector {
  // 字符串规则 根据逗号分割
  pub paradigm_vec: Vec<Vec<SelectParadigm>>,

  // 坐标位置
  pub loc: Option<Loc>,

  // 内部处理 地图
  #[serde(skip_serializing)]
  map: LocMap,

  // 字符串 操作 序列
  #[serde(skip_serializing)]
  pub charlist: Vec<char>,

  // 节点 父节点
  // 延迟赋值
  #[serde(skip_serializing)]
  pub parent: NodeWeakRef,
}

impl NewSelector {
  pub fn new(
    charlist: Vec<char>,
    loc: Option<Loc>,
    map: Option<LocMap>,
    parent: NodeWeakRef,
  ) -> HandleResult<Self> {
    let mut obj = NewSelector {
      paradigm_vec: vec![],
      loc,
      map: map.unwrap_or_else(|| LocMap::new(&charlist)),
      charlist,
      parent,
    };
    match obj.parse() {
      Ok(()) => HandleResult::Success(obj),
      Err(msg) => HandleResult::Fail(msg),
    }
  }

  ///
  /// 尽量减少调用次数
  ///
  pub fn value(&self) -> String {
    self.charlist.poly()
  }

  ///
  /// 打印错误信息
  ///
  pub fn errormsg(&self, index: &usize) -> Result<(), String> {
    let char = *self.charlist.get(*index).unwrap();
    let error_loc = self.map.get(index).unwrap();
    Err(format!(
      "select text {}, char {} is not allow, line is {} col is {}",
      self.charlist.poly(),
      char,
      error_loc.line,
      error_loc.col
    ))
  }

  ///
  /// 在二维数组 的最后 追加 词
  ///
  pub fn add_paradigm(&mut self, obj: SelectParadigm) {
    if self.paradigm_vec.is_empty() {
      self.paradigm_vec.push(vec![obj]);
    } else {
      let list = self.paradigm_vec.last_mut().unwrap();
      list.push(obj);
    }
  }

  ///
  /// 最后一组词 的 最后一位 非空
  /// 逗号情况 调用
  ///
  pub fn clear_paraigm(&mut self, index: &usize) -> Result<(), String> {
    if let Some(list) = self.paradigm_vec.last_mut() {
      let mut rm_index_list: Vec<usize> = vec![];
      let mut num = 0;
      for (index, word) in list.iter().rev().enumerate() {
        match word {
          SelectParadigm::CominaWrap(_) => {
            rm_index_list.push(list.len() - 1 - index);
          }
          _ => {
            break;
          }
        }
      }
      for (_, val) in rm_index_list.into_iter().enumerate() {
        list.remove(val - num);
        num += 1;
      }
      if list.is_empty() {
        return Err(self.errormsg(index).err().unwrap());
      }
      Ok(())
    } else {
      Err(self.errormsg(index).err().unwrap())
    }
  }

  ///
  /// 在二维数组中 开辟 一组 新词 序列
  ///
  pub fn add_paradigm_vec(&mut self) {
    self.paradigm_vec.push(vec![]);
  }

  ///
  /// 获取 最后 词
  ///
  pub fn last_paradigm(&self) -> Option<&SelectParadigm> {
    if self.paradigm_vec.last().is_some() {
      return self.paradigm_vec.last().unwrap().last();
    }
    None
  }

  ///
  /// 获取 最后 非空格 词
  ///
  pub fn last_paradigm_without_space(&self) -> Option<&SelectParadigm> {
    if let Some(list) = self.paradigm_vec.last() {
      for p in list.iter().rev() {
        if !matches!(p, SelectParadigm::CominaWrap(..))
          || *p != SelectParadigm::CominaWrap(TokenCombinaChar::Space)
        {
          return Some(p);
        } else {
          continue;
        }
      }
      None
    } else {
      None
    }
  }

  ///
  /// 是否 停词 的判断
  ///
  pub fn is_end(char: Option<&char>, extend_char: Option<Vec<char>>) -> bool {
    if let Some(cc) = char {
      let mut charlist: Vec<char> = vec![];
      if let Some(mut extend_list) = extend_char {
        charlist.append(&mut extend_list);
      }
      charlist.append(
        &mut TokenSelectChar::iterator()
          .map(|x| x.to_str())
          .collect::<Vec<char>>(),
      );
      charlist.append(
        &mut TokenCombinaChar::iterator()
          .map(|x| x.to_str())
          .collect::<Vec<char>>(),
      );
      charlist.append(
        &mut TokenKeyWordChar::iterator()
          .map(|x| x.to_str())
          .collect::<Vec<char>>(),
      );
      charlist.contains(cc)
    } else {
      false
    }
  }

  ///
  /// parse select txt
  /// https://www.w3schools.com/cssref/css_selectors.asp
  ///
  pub fn parse(&mut self) -> Result<(), String> {
    let charlist = &self.charlist.clone();
    let index: usize = 0;

    let (_, end) = traversal(
      Some(index),
      charlist,
      &mut (|arg, charword| {
        let (index, _, _) = arg;
        let (_, char, _) = charword;

        if Token::is_token(Some(char)) {
          if TokenSelectChar::is(char) {
            // example a, li , h2
            let (select_word, end) = self.parse_selector_word(index)?;
            self.add_paradigm(SelectParadigm::SelectWrap(select_word));
            *index = end;
          } else if TokenCombinaChar::is(char) {
            let (_, end) = self.parse_combina_word(index)?;
            *index = end;
          } else if TokenKeyWordChar::is(char) {
          } else if TokenAllowChar::is(char) {
            // example a, li , h2
            let (select_word, end) = self.parse_selector_word(index)?;
            self.add_paradigm(SelectParadigm::SelectWrap(select_word));
            *index = end;
          }
        } else {
          // example a, li , h2
          let (select_word, end) = self.parse_selector_word(index)?;
          self.add_paradigm(SelectParadigm::SelectWrap(select_word));
          *index = end;
        }
        Ok(())
      }),
    )?;
    self.clear_paraigm(&end)?;
    Ok(())
  }

  ///
  /// 连接词的处理
  ///
  fn parse_combina_word(&mut self, index: &usize) -> Result<(String, usize), String> {
    let char = *self.charlist.get(*index).unwrap();
    if Token::is_space_token(Some(&char)) {
      let last_pardigm = self.last_paradigm();
      if let Some(SelectParadigm::CominaWrap(token)) = last_pardigm {
        if *token != TokenCombinaChar::Space {
          self.add_paradigm(SelectParadigm::CominaWrap(TokenCombinaChar::Space));
        }
      } else {
        self.add_paradigm(SelectParadigm::CominaWrap(TokenCombinaChar::Space));
      }
    } else if char == TokenCombinaChar::AddChar.to_str()
      || char == TokenCombinaChar::ExtendChar.to_str()
      || char == TokenCombinaChar::BrotherMatchChar.to_str()
      || char == TokenCombinaChar::ColumnChar.to_str()
    {
      let last_pardigm = self.last_paradigm_without_space();
      // 只要最后一个非空字符是 非链接符 即可
      if !matches!(last_pardigm, Some(SelectParadigm::CominaWrap(..))) {
        let combin_token = TokenCombinaChar::get(&char).unwrap();
        self.add_paradigm(SelectParadigm::CominaWrap(combin_token));
      } else {
        return Err(self.errormsg(index).err().unwrap());
      }
    } else if char == TokenCombinaChar::Comma.to_str() {
      self.clear_paraigm(index)?;
      self.add_paradigm_vec();
    }
    Ok((char.to_string(), *index + 1))
  }

  ///
  /// parse select word
  ///
  fn parse_selector_word(&mut self, start: &usize) -> Result<(String, usize), String> {
    let res: (String, usize);
    let charlist = &self.charlist;
    let char = charlist.get(*start).unwrap();
    if *char == '.' || *char == '#' {
      res = self.parse_selector_class_or_id_word(start)?;
    } else if *char == ':' {
      res = self.parse_selector_pseudo_class_word(start)?;
    } else if *char == '*' {
      res = ('*'.to_string(), *start + 1);
    } else if *char == '[' {
      res = self.parse_selector_attr(start)?;
    } else if *char == '(' {
      res = self.parse_selector_brackets(start)?;
    } else if !Token::is_token(Some(char)) || TokenAllowChar::is(char) {
      res = self.parse_selector_ele(start)?;
    } else {
      return Err(self.errormsg(start).err().unwrap());
    }
    Ok(res)
  }

  ///
  /// parse example h2 span div
  ///
  fn parse_selector_ele(&mut self, start: &usize) -> Result<(String, usize), String> {
    traversal(
      Some(*start),
      &self.charlist,
      &mut (|arg, charword| {
        let (index, temp, end) = arg;
        let (_, char, _) = charword;
        if Token::is_token(Some(char)) {
          if TokenAllowChar::is(char) {
            temp.push(*char);
            return Ok(());
          }
          if Self::is_end(Some(char), None) {
            *end = true;
          } else {
            return Err(self.errormsg(index).err().unwrap());
          }
        } else {
          temp.push(*char);
        }
        Ok(())
      }),
    )
  }

  ///
  /// parse example #h2 .abc
  ///
  fn parse_selector_class_or_id_word(&mut self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let mut record = false;
    traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, temp, end) = arg;
        let (_, char, _) = charword;
        if Token::is_token(Some(char)) {
          if TokenAllowChar::is(char) {
            temp.push(*char);
            return Ok(());
          }
          if TokenSelectChar::is(char) && !record {
            record = true;
            temp.push(*char);
            return Ok(());
          }
          if Self::is_end(Some(char), None) && record {
            if temp.len() < 2 {
              // . , # single word is error
              return Err(self.errormsg(index).err().unwrap());
            }
            *end = true;
          } else {
            return Err(self.errormsg(index).err().unwrap());
          }
        } else {
          temp.push(*char);
        }
        Ok(())
      }),
    )
  }

  ///
  /// parse example ::hide :next
  ///
  fn parse_selector_pseudo_class_word(&mut self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let mut record = false;
    traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, temp, end) = arg;
        let (_, char, next) = charword;
        if Token::is_token(Some(char)) {
          if !record && *char == ':' {
            if next == Some(&':') {
              temp.push(*char);
              *index += 1;
            }
            temp.push(*char);
            record = true;
            return Ok(());
          }
          if TokenAllowChar::is(char) {
            temp.push(*char);
            return Ok(());
          }
          if Self::is_end(Some(char), None) && record {
            if (temp.len() < 2 && *temp == vec![':']) || (temp.len() < 3 && *temp == vec![':', ':'])
            {
              // . , # single word is error
              return Err(self.errormsg(index).err().unwrap());
            }
            *end = true;
          } else {
            return Err(self.errormsg(index).err().unwrap());
          }
        } else {
          temp.push(*char);
        }
        Ok(())
      }),
    )
  }

  ///
  /// parse example (language)
  ///
  fn parse_selector_brackets(&mut self, start: &usize) -> Result<(String, usize), String> {
    let mut level = 0;
    let res = traversal(
      Some(*start),
      &self.charlist,
      &mut (|arg, charword| {
        let (index, temp, end) = arg;
        let (_, char, next) = charword;
        if Token::is_token(Some(char)) {
          if TokenAllowChar::is(char) {
            temp.push(*char);
            return Ok(());
          }
          if *char == ')' {
            level -= 1;
            if level < 0 {
              return Err(self.errormsg(index).err().unwrap());
            } else if level == 0 {
              *end = true;
              return Ok(());
            } else if *char == '@' && next != Some(&'{') {
              return Err(self.errormsg(index).err().unwrap());
            }
          }
          if *char == '(' {
            level += 1;
          }
          temp.push(*char);
        } else {
          temp.push(*char);
        }
        Ok(())
      }),
    )?;
    if level > 0 {
      return Err(self.errormsg(&res.1).err().unwrap());
    }
    Ok(res)
  }

  ///
  /// parse example '[arco-theme='dark']'
  ///
  fn parse_selector_attr(&mut self, start: &usize) -> Result<(String, usize), String> {
    // example ~= *= ^=
    let equal_chars = vec!['~', '|', '^', '$', '*'];
    let mut hasequal = false;
    let mut has_brackest = false;
    let mut queto: Option<char> = None;

    let res = traversal(
      Some(*start),
      &self.charlist,
      &mut (|arg, charword| {
        let (index, temp, end) = arg;
        let (prev, char, next) = charword;
        if let Some(token_queto) = queto {
          if has_brackest {
            if *char == token_queto && prev != Some(&'\\') {
              queto = None;
            }
            temp.push(*char);
          } else {
            return Err(self.errormsg(index).err().unwrap());
          }
        } else if Token::is_token(Some(char)) {
          if has_brackest {
            if *char == '[' {
              return Err(self.errormsg(index).err().unwrap());
            } else if *char == ']' {
              let prev_char_without_space = {
                let mut res: Option<char> = None;
                for tc in temp.iter().rev() {
                  if *tc == ' ' {
                    continue;
                  } else {
                    res = Some(*tc);
                    break;
                  }
                }
                res
              };
              if prev_char_without_space == Some('=') {
                return Err(self.errormsg(index).err().unwrap());
              }
              // is attr end record
              temp.push(*char);
              has_brackest = false;
              *end = true;
            } else if TokenAllowChar::is(char) {
              temp.push(*char);
            } else if equal_chars.contains(char) {
              if next == Some(&'=') && !hasequal {
                temp.push(*char);
                temp.push('=');
                hasequal = true;
                *index += 1;
              } else {
                return Err(self.errormsg(index).err().unwrap());
              }
            } else if *char == '=' {
              if !hasequal {
                temp.push('=');
                hasequal = true;
              } else {
                return Err(self.errormsg(index).err().unwrap());
              }
            }
          } else {
            // start record attr
            if *char == '[' {
              temp.push(*char);
              has_brackest = true;
            } else {
              return Err(self.errormsg(index).err().unwrap());
            }
          }
        } else if has_brackest {
          temp.push(*char);
        } else {
          return Err(self.errormsg(index).err().unwrap());
        }
        Ok(())
      }),
    )?;
    if has_brackest {
      return Err(self.errormsg(&res.1).err().unwrap());
    }
    Ok(res)
  }
}
