use crate::new_less::loc::Loc;
use crate::new_less::node::{HandleResult, NodeWeakRef};
use crate::new_less::option::ParseOption;
use serde::Serialize;
use std::ops::Deref;
use crate::new_less::fileinfo::FileWeakRef;

#[derive(Debug, Clone, Serialize)]
pub struct StyleRuleNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Option<Loc>,

  // 自身 Rule 的弱引用
  #[serde(skip_serializing)]
  parent: NodeWeakRef,

  // 文件信息
  #[serde(skip_serializing)]
  pub fileinfo: FileWeakRef,
}

impl StyleRuleNode {
  pub fn new(txt: String, loc: Option<Loc>, parent: NodeWeakRef, fileinfo: FileWeakRef) -> HandleResult<Self> {
    let obj = Self {
      content: txt,
      loc,
      parent,
      fileinfo
    };
    HandleResult::Success(obj)
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
}
