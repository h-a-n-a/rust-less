use crate::extend::enum_extend::EnumExtend;
use crate::extend::string::StringExtend;
use crate::new_less::fileinfo::FileWeakRef;
use crate::new_less::ident::IdentType;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::NodeWeakRef;
use crate::new_less::scan::{traversal, ScanArg, ScanResult};
use crate::new_less::token::lib::Token;
use crate::new_less::token::value::TokenValueAllow;
use serde::Serialize;
use std::fmt::{Debug, Formatter};

#[derive(Serialize, Clone)]
pub struct ValueNode {
  // 原始字符
  pub origin_txt: String,

  // 字符 向量 只读
  #[serde(skip_serializing)]
  charlist: Vec<String>,

  // rule 父节点
  #[serde(skip_serializing)]
  pub parent: NodeWeakRef,

  // 文件节点
  #[serde(skip_serializing)]
  pub fileinfo: FileWeakRef,

  // 内部处理 地图
  #[serde(skip_serializing)]
  map: LocMap,

  // 单词 范式
  pub word_ident_list: Vec<IdentType>,
}

impl Debug for ValueNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ValueNode")
      .field("origin_txt", &self.origin_txt)
      .field("word_ident_list", &self.word_ident_list)
      .finish()
  }
}

impl ValueNode {
  pub fn new(
    txt: String,
    loc: Option<Loc>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
  ) -> Result<Self, String> {
    let map = if loc.is_none() {
      LocMap::new(txt.clone())
    } else {
      LocMap::merge(loc.as_ref().unwrap(), &txt).0
    };
    let mut obj = Self {
      origin_txt: txt.clone(),
      charlist: txt.tocharlist(),
      parent,
      fileinfo,
      map,
      word_ident_list: vec![],
    };
    obj.parse()?;
    Ok(obj)
  }

  ///
  /// 报错信息
  ///
  pub fn error_msg(&self, index: &usize) -> String {
    let error_loc = self.map.get(index).unwrap();
    let char = self.charlist.get(*index).unwrap().to_string();
    format!(
      "text {}, char {} is not allow, line is {} col is {}",
      &self.origin_txt, char, error_loc.line, error_loc.col
    )
  }

  ///
  /// 产生代码
  ///
  pub fn code_gen(&self) {}

  ///
  /// 是否是数字
  ///
  pub fn is_number(char: &str) -> bool {
    char.parse::<i32>().is_ok()
  }

  pub fn is_end(char: &str, extend_char: Option<Vec<&str>>) -> bool {
    let mut char_list = vec![";", "@", "~", "#", "$", "(", ")", "[", "]", "+", "*", "/"];
    if let Some(mut extend_list) = extend_char {
      char_list.append(&mut extend_list);
    }
    Token::is_space_token(char) || char_list.contains(&char)
  }

  ///
  /// 转化 常用词 颜色 关键词
  ///
  pub fn parse_value_word(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let res = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let ScanArg {
          mut temp,
          index,
          mut hasend,
        } = arg;
        let (_, char, nextchar) = charword;
        temp += &char;
        if &char == ":" {
          return Err(self.error_msg(&index));
        }
        if Self::is_end(&nextchar, None) {
          hasend = true;
        }
        Ok(ScanResult::Arg(ScanArg {
          index,
          temp,
          hasend,
        }))
      }),
    )?;
    Ok(res)
  }

  ///
  /// 转化 引号词
  ///
  pub fn parse_value_string_const(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let mut keyword: String = "".to_string();
    let (value, end) = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let ScanArg {
          mut temp,
          mut index,
          mut hasend,
        } = arg;
        let (_, char, nextchar) = charword;
        // todo @{...} not support
        if temp.is_empty() {
          if &char == r#"'"# || &char == r#"""# {
            keyword = char.clone();
            temp += &char;
          } else {
            return Err(self.error_msg(&index));
          }
        } else {
          temp += &char;
        }

        if nextchar == keyword && char != r#"\"# {
          hasend = true;
          temp += &keyword;
          index += 1;
        }

        Ok(ScanResult::Arg(ScanArg {
          index,
          temp,
          hasend,
        }))
      }),
    )?;

    // 最终检查
    if value.is_empty() {
      return Err(self.error_msg(start));
    }
    if (value.len() > 1
      && *value.tocharlist().get(0).unwrap() == keyword
      && *value.tocharlist().get(value.len() - 1).unwrap() != keyword)
      || value.len() == 1
    {
      return Err(format!("{} is not closure", self.origin_txt));
    }

    Ok((value, end))
  }

  ///
  /// 转化变量
  ///
  pub fn parse_value_var(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let res = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let ScanArg {
          mut temp,
          mut index,
          mut hasend,
        } = arg;
        let (_, char, nextchar) = charword;
        // 第一位必须是 @
        if temp.is_empty() && &char == "@" {
          temp += "@";
          Ok(ScanResult::Arg(ScanArg {
            index,
            temp,
            hasend,
          }))
        } else if temp.is_empty() {
          Err(self.error_msg(&index))
        } else {
          // 后续写词
          if Token::is_token(&char) {
            if &char == "-" {
              if Token::is_token(&nextchar) {
                hasend = true;
                index -= 1;
              } else if !nextchar.is_empty() {
                temp += &char;
              }
              // @- is error
              if temp.len() < 2 {
                return Err(self.error_msg(&index));
              }
            } else if Self::is_end(&char, None) {
              // @+ @* is error
              if temp.len() < 2 {
                return Err(self.error_msg(&index));
              }
              hasend = true;
              index -= 1;
            } else if &char == r#"\"# {
              temp += &char;
            } else {
              return Err(self.error_msg(&index));
            }
          } else {
            temp += &char;
          }
          Ok(ScanResult::Arg(ScanArg {
            index,
            temp,
            hasend,
          }))
        }
      }),
    )?;
    Ok(res)
  }

  ///
  /// 转化 数值
  ///
  pub fn parse_value_number(
    &self,
    start: &usize,
  ) -> Result<((String, Option<String>), usize), String> {
    let charlist = &self.charlist;
    let mut value: String = "".to_string();
    let mut unit: String = "".to_string();
    let mut has_record_value = false;

    let (_, end) = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let ScanArg {
          temp,
          mut index,
          mut hasend,
        } = arg;
        let (_, char, nextchar) = charword;

        if Token::is_token(&char) {
          return Err(self.error_msg(&index));
        } else if Self::is_number(&char) {
          if !has_record_value {
            value += &char;
          } else {
            index -= 1;
            hasend = true;
          }
        } else {
          if value.is_empty() {
            return Err(self.error_msg(&index));
          }
          if !has_record_value {
            has_record_value = true;
          }
          unit += &char;
        }

        if Self::is_end(&nextchar, Some(vec!["-"])) {
          hasend = true;
        }

        Ok(ScanResult::Arg(ScanArg {
          index,
          temp,
          hasend,
        }))
      }),
    )?;
    if unit.is_empty() {
      Ok(((value, None), end))
    } else {
      Ok(((value, Some(unit)), end))
    }
  }

  ///
  /// 判断 是否 是 操作符
  ///
  fn is_operator(char: &str) -> bool {
    vec!["+", "-", "*", "/"].contains(&char)
  }

  ///
  /// 检测 中小 括号 是否能够对齐
  ///
  fn validate_brackets() -> Box<dyn FnMut(&str) -> Result<Vec<String>, String>> {
    let mut brackets_vaildate: Vec<String> = vec![];
    Box::new(move |char: &str| {
      if TokenValueAllow::is(char) {
        if char == "]" || char == ")" {
          let last = brackets_vaildate.last();
          if let Some(last_char) = last {
            if (last_char == "(" && char == ")") || (last_char == "[" && char == "]") {
              brackets_vaildate.remove(brackets_vaildate.len() - 1);
            } else {
              return Err(format!(r#"{} is error "#, char));
            }
          } else {
            return Err(format!(r#"{} is error "#, char));
          }
        } else {
          brackets_vaildate.push(char.to_string())
        }
      } else {
        return Err(format!(r#"{} is not '(' ')' '[' ']' "#, char));
      }
      Ok(brackets_vaildate.clone())
    })
  }

  ///
  /// 转化
  ///
  fn parse(&mut self) -> Result<(), String> {
    let charlist = self.charlist.clone();
    if charlist.is_empty() {
      return Err("var declare text is empty".to_string());
    }
    let index: usize = 0;
    let mut validate_fn = Self::validate_brackets();
    let mut vaildate_res: Vec<String> = vec![];

    traversal(
      Some(index),
      &charlist,
      &mut (|arg, charword| {
        let ScanArg {
          temp,
          mut index,
          hasend,
        } = arg;
        let (_, char, _) = charword;

        // 处理空格
        if Token::is_space_token(&char) {
          match self.word_ident_list.last() {
            None => {}
            Some(val) => match val {
              IdentType::Space => {
                return Ok(ScanResult::Skip);
              }
              _ => {
                self.word_ident_list.push(IdentType::Space);
              }
            },
          }
        } else if &char == "@" {
          let (var, end) = self.parse_value_var(&index)?;
          self.word_ident_list.push(IdentType::Var(var));
          index = end;
        }
        // 处理结尾词 ignore
        else if &char == r#";"# {
          return if index == self.charlist.len() - 1 {
            Ok(ScanResult::Skip)
          } else {
            Err(self.error_msg(&index))
          };
        }
        // 处理prop
        else if &char == "$" || &char == "~" {
          // todo! $ style_rule
          // todo! ~ reference
          return Err(format!(
            "$ style_rule or ~ reference has not support \n {}",
            self.error_msg(&index)
          ));
        }
        // 处理 引用
        else if &char == "#" {
          let (color, end) = self.parse_value_word(&index)?;
          self.word_ident_list.push(IdentType::Color(color));
          index = end;
        }
        // 处理 keyword
        else if &char == "!" {
          let end = index + 10;
          if self.charlist.len() >= end && &self.charlist[index..end].join("") == "!important" {
            self
              .word_ident_list
              .push(IdentType::KeyWord("!important".to_string()));
            index += 10;
          } else {
            let (word, end) = self.parse_value_word(&index)?;
            self.word_ident_list.push(IdentType::Word(word));
            index = end;
          }
        }
        // 处理引号词
        else if &char == r#"""# || &char == r#"'"# {
          let (string_const, end) = self.parse_value_string_const(&index)?;
          self
            .word_ident_list
            .push(IdentType::StringConst(string_const));
          index = end;
        }
        // 处理括号
        else if TokenValueAllow::is(&char) {
          if &char != r#"\"# {
            match validate_fn(&char) {
              Ok(res) => {
                vaildate_res = res;
                self.word_ident_list.push(IdentType::Brackets(char));
              }
              Err(..) => {
                return Err(self.error_msg(&index));
              }
            };
          } else {
            return Ok(ScanResult::Skip);
          }
        }
        // 操作符
        else if Self::is_operator(&char) {
          if self.word_ident_list.is_empty() {
            return Ok(ScanResult::Skip);
          } else {
            let last_item = self.word_ident_list.last().unwrap();
            if last_item.is_number() || last_item.is_var() {
              self.word_ident_list.push(IdentType::Operator(char));
            } else if last_item.is_space() {
              if self.word_ident_list.len() > 1 {
                let before_last_item = self
                  .word_ident_list
                  .get(self.word_ident_list.len() - 2)
                  .unwrap();
                if before_last_item.is_var() || before_last_item.is_number() {
                  self.word_ident_list.push(IdentType::Operator(char));
                } else if before_last_item.is_operator() {
                  return Err(self.error_msg(&index));
                } else {
                  return Ok(ScanResult::Skip);
                }
              } else {
                return Ok(ScanResult::Skip);
              }
            } else {
              self.word_ident_list.push(IdentType::Space);
            }
          }
        }
        // 处理 数值
        else if Self::is_number(&char) {
          let ((val, unit), end) = self.parse_value_number(&index)?;
          self.word_ident_list.push(IdentType::Number(val, unit));
          index = end;
        }
        // 处理单词
        else {
          let (word, end) = self.parse_value_word(&index)?;
          self.word_ident_list.push(IdentType::Word(word));
          index = end;
        }
        let new_arg = ScanArg {
          index,
          temp,
          hasend,
        };
        Ok(ScanResult::Arg(new_arg))
      }),
    )?;

    // 括号有没有闭合的情况
    if !vaildate_res.is_empty() {
      return Err(format!(
        "{} contains unclosed parentheses -> {:#?}",
        &self.origin_txt, &vaildate_res
      ));
    }

    Ok(())
  }
}
