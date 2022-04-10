use crate::extend::vec_str::VecCharExtend;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::NodeWeakRef;
use crate::new_less::scan::traversal;
use crate::new_less::var::HandleResult;
use serde::Serialize;
use crate::new_less::token::lib::{Token, TokenInterface};
use crate::new_less::token::new_select::{TokenAllowChar, TokenCombinaChar, TokenKeyWordChar, TokenSelectChar};

#[derive(Debug, Clone, Serialize)]
pub enum SelectParadigm {
  SelectWrap(String),
  CominaWrap(String),
  VarWrap(String),
}

pub trait Paradigm {
  fn join(&self) -> String;
}

impl Paradigm for Vec<SelectParadigm> {
  fn join(&self) -> String {
    let mut txt = "".to_string();
    self.iter().for_each(|par| {
      match par {
        SelectParadigm::SelectWrap(cc) |
        SelectParadigm::CominaWrap(cc) |
        SelectParadigm::VarWrap(cc) => {
          txt += cc
        }
      }
    });
    txt
  }
}


#[derive(Debug, Clone, Serialize)]
pub struct NewSelector {
  // 字符串规则 根据逗号分割
  pub paradigm_vec: Vec<SelectParadigm>,

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

  pub fn value(&self) -> String {
    self.charlist.poly()
  }

  ///
  /// 打印错误信息
  ///
  pub fn errormsg(&mut self, index: &usize) -> Result<(), String> {
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

  pub fn is_end(char: Option<&char>, extend_char: Option<Vec<char>>) -> bool {
    if let Some(cc) = char {
      let mut charlist: Vec<char> = vec![];
      if let Some(mut extend_list) = extend_char {
        charlist.append(&mut extend_list);
      }
      charlist.append(&mut TokenAllowChar::iterator()
        .map(|x| x.to_str())
        .collect::<Vec<char>>()
      );
      charlist.append(&mut TokenCombinaChar::iterator()
        .map(|x| x.to_str())
        .collect::<Vec<char>>()
      );
      charlist.append(&mut TokenKeyWordChar::iterator()
        .map(|x| x.to_str())
        .collect::<Vec<char>>()
      );
      charlist.contains(cc)
    } else {
      false
    }
  }


  ///
  /// parse select txt
  ///
  pub fn parse(&mut self) -> Result<(), String> {
    let charlist = &self.charlist.clone();
    let index: usize = 0;
    traversal(
      Some(index),
      charlist,
      &mut (|arg, charword| {
        let (index, _, _) = arg;
        let (_, char, _) = charword;
        if Token::is_token(Some(char)) {
          if TokenSelectChar::is(char) {
            // example a, li , h2
            let (select_word, end) = self.parse_select(&index)?;
            self.paradigm_vec.push(SelectParadigm::SelectWrap(select_word));
            *index = end;
          } else if TokenCombinaChar::is(char) {} else if TokenKeyWordChar::is(char) {} else if TokenAllowChar::is(char) {
            // example a, li , h2
            let (select_word, end) = self.parse_select(&index)?;
            self.paradigm_vec.push(SelectParadigm::SelectWrap(select_word));
            *index = end;
          }
        } else {
          // example a, li , h2
          let (select_word, end) = self.parse_select(&index)?;
          self.paradigm_vec.push(SelectParadigm::SelectWrap(select_word));
          *index = end;
        }
        Ok(())
      }))?;
    Ok(())
  }

  fn parse_select(&mut self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let res = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (_, _, _) = arg;
        let (_, char, _) = charword;
        if Token::is_token(Some(char)) {} else {}
        Ok(())
      }))?;
    Ok(res)
  }
}