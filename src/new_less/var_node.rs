use std::ops::Deref;
use crate::extend::string::StringExtend;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{HandleResult, NodeWeakRef};
use serde::Serialize;
use crate::new_less::option::ParseOption;

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
      Ok(_) => {
        HandleResult::Success(obj)
      }
      Err(msg) => {
        HandleResult::Fail(msg)
      }
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
      None => {
        Default::default()
      }
      Some(pr) => {
        match pr.upgrade().unwrap().deref().borrow().file_info.clone() {
          None => {
            Default::default()
          }
          Some(file) => {
            file.upgrade().unwrap().deref().borrow().option.clone()
          }
        }
      }
    }
  }

  ///
  /// 报错信息
  ///
  pub fn error_msg(&self, index: &usize) -> String {
    let error_loc = self.map.get(index).unwrap();
    let char = self.charlist.get(*index).unwrap();
    format!("text {}, char {} is not allow, line is {} col is {}", &self.content, char, error_loc.line, error_loc.col)
  }


  ///
  /// 转化校验
  ///
  fn parse(&self) -> Result<(), String> {
    Ok(())
  }
}


///
/// 检查是否 合规
/// 检查是否 变量
///
pub fn is_var(charlist: &[String], istop: bool, locationmsg: String) -> Result<bool, String> {
  // 变量片段中 含有换行
  if charlist.is_empty() {
    return Err(format!("var token word is empty,{}", locationmsg));
  }
  if charlist
    .iter()
    .filter(|&x| x.as_str() == "\n" || x.as_str() == "\r")
    .count()
    > 0
  {
    return Err(format!(
      r#"token word has contains "\n","\r",{} "#,
      locationmsg
    ));
  }
  // 变量类似 ;; || @a:10px;;
  if charlist[0].as_str() == ";" {
    return Err(format!(r#"token word is only semicolon,{} "#, locationmsg));
  }
  if istop {
    // 变量片段中首位必须是 @
    if charlist[0].as_str() != "@" {
      return Err(format!(
        r#"token word is not with @ begin,{} "#,
        locationmsg
      ));
    }
    // 先判断 是否含有 @import
    if charlist.join("").indexOf("@import", None) > -1 {
      return Ok(false);
    }
    // 判断是否复合基本格式
    if charlist.join("").split(':').count() != 2 {
      return Err(format!(
        r#"var token is not liek '@var: 10px',{} ,{}"#,
        charlist.join(""),
        locationmsg
      ));
    }
  }
  Ok(true)
}
