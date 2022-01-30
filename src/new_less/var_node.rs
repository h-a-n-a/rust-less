use crate::extend::string::StringExtend;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{HandleResult, NodeWeakRef};
use crate::new_less::option::ParseOption;
use crate::new_less::scan::{traversal, ScanArg, ScanResult};
use crate::new_less::token::lib::Token;
use serde::Serialize;
use std::ops::Deref;

#[derive(Debug, Clone, Serialize)]
pub struct VarNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Option<Loc>,

  // 内部处理 地图
  #[serde(skip_serializing)]
  map: LocMap,

  // 字符串 操作 序列
  #[serde(skip_serializing)]
  charlist: Vec<String>,

  // 节点 父节点
  #[serde(skip_serializing)]
  pub parent: NodeWeakRef,
}

impl VarNode {
  ///
  /// 初始化
  ///
  pub fn new(txt: String, loc: Option<Loc>, parent: NodeWeakRef) -> HandleResult<Self> {
    let map: LocMap;
    if loc.is_none() {
      map = LocMap::new(txt.clone())
    } else {
      map = LocMap::merge(&loc.as_ref().unwrap(), &txt).0;
    }
    let obj = Self {
      content: txt.clone(),
      loc,
      map,
      charlist: txt.tocharlist(),
      parent,
    };
    if !obj.content.is_empty() && obj.charlist.get(0).unwrap() != "@" && !obj.is_top() {
      return HandleResult::Swtich;
    } else if !obj.content.is_empty() && obj.charlist.get(0).unwrap() != "@" && obj.is_top() {
      return HandleResult::Fail(obj.error_msg(&0));
    }
    match obj.parse() {
      Ok(_) => HandleResult::Success(obj),
      Err(msg) => HandleResult::Fail(msg),
    }
  }

  pub fn is_top(&self) -> bool {
    self.parent.is_none()
  }

  ///
  /// 获取选项
  ///
  pub fn get_options(&self) -> ParseOption {
    match self.parent.clone() {
      None => Default::default(),
      Some(pr) => match pr.upgrade().unwrap().deref().borrow().file_info.clone() {
        None => Default::default(),
        Some(file) => file.upgrade().unwrap().deref().borrow().option.clone(),
      },
    }
  }

  ///
  /// 报错信息
  ///
  pub fn error_msg(&self, index: &usize) -> String {
    let error_loc = self.map.get(index).unwrap();
    let char = self.charlist.get(*index).unwrap();
    format!(
      "text {}, char {} is not allow, line is {} col is {}",
      &self.content, char, error_loc.line, error_loc.col
    )
  }

  pub fn parse_var_ident(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    match traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let ScanArg {
          mut temp,
          mut index,
          mut hasend,
        } = arg;
        let (_, char, next) = charword;
        if Token::is_token(&char) {
        } else {
          temp += &char;
        }
        let new_arg = ScanArg {
          index,
          temp,
          hasend,
        };
        Ok(ScanResult::Arg(new_arg))
      }),
    ) {
      Ok(obj) => Ok(obj),
      Err(msg) => Err(msg),
    }
  }

  ///
  /// 转化校验
  ///
  fn parse(&self) -> Result<(), String> {
    let charlist = &self.charlist;
    if charlist.is_empty() {
      return Err("var declare text is empty".to_string());
    }
    let mut word_vec = vec!["@".to_string()];
    let index = 1;

    match traversal(
      Some(index),
      charlist,
      &mut (|arg, charword| {
        let mut temp = arg.temp;
        let mut index = arg.index;
        let (key, jump) = self.parse_var_ident(&arg.index)?;
        index = jump;
        temp += &key;
        let new_arg = ScanArg {
          index,
          temp,
          hasend: false,
        };
        Ok(ScanResult::Arg(new_arg))
      }),
    ) {
      Ok(res) => {}
      Err(msg) => {
        return Err(msg);
      }
    };

    Ok(())
  }
}
