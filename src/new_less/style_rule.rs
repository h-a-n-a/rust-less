use crate::extend::string::StringExtend;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::FileWeakRef;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{HandleResult, NodeWeakRef};
use crate::new_less::option::ParseOption;
use crate::new_less::value::ValueNode;
use serde::Serialize;
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct StyleRuleNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Option<Loc>,

  // 字符串 操作 序列
  #[serde(skip_serializing)]
  charlist: Vec<String>,

  // uuid 避免 查找时循环引用
  pub uuid: String,

  // 内部处理 地图
  #[serde(skip_serializing)]
  map: LocMap,

  // 文件信息
  #[serde(skip_serializing)]
  pub fileinfo: FileWeakRef,

  // 节点 父节点
  #[serde(skip_serializing)]
  pub parent: NodeWeakRef,

  // 上下文
  #[serde(skip_serializing)]
  pub context: ParseContext,

  // 对应的 key 值
  pub key: Option<String>,

  // 对应 值
  pub value: Option<ValueNode>,
}

impl Debug for StyleRuleNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ValueNode")
      .field("content", &self.content)
      .field("loc", &self.loc)
      .field("uuid", &self.uuid)
      .field("key", &self.key)
      .field("value", &self.value)
      .finish()
  }
}

impl StyleRuleNode {
  pub fn new(
    txt: String,
    loc: Option<Loc>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
    context: ParseContext,
  ) -> HandleResult<Self> {
    let map = if loc.is_none() {
      LocMap::new(txt.clone())
    } else {
      LocMap::merge(loc.as_ref().unwrap(), &txt).0
    };
    let mut obj = Self {
      content: txt.clone(),
      loc,
      charlist: txt.tocharlist(),
      uuid: Uuid::new_v4().to_string(),
      map,
      fileinfo,
      parent,
      context,
      key: None,
      value: None,
    };
    match obj.parse() {
      Ok(_) => HandleResult::Success(obj),
      Err(msg) => HandleResult::Fail(msg),
    }
  }

  ///
  /// 获取选项
  ///
  pub fn get_options(&self) -> ParseOption {
    self.context.borrow().option.clone()
  }

  ///
  /// 判断是否是 顶层 节点 下的变量
  ///
  pub fn is_top(&self) -> bool {
    self.parent.is_none()
  }

  ///
  /// 报错信息
  ///
  pub fn error_msg(&self, index: &usize) -> String {
    let error_loc = self.map.get(index).unwrap();
    let char = self.charlist.get(*index).unwrap().to_string();
    format!(
      "text {}, char {} is not allow, line is {} col is {}",
      &self.content, char, error_loc.line, error_loc.col
    )
  }

  fn parse(&mut self) -> Result<(), String> {
    let charlist = &self.charlist.clone();
    if charlist.is_empty() {
      return Err("var declare text is empty".to_string());
    }
    Ok(())
  }
}
