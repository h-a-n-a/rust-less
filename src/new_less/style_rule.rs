use crate::new_less::loc::Loc;
use crate::new_less::node::{HandleResult, NodeWeakRef};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StyleRuleNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Option<Loc>,

  // 自身 Rule 的弱引用
  #[serde(skip_serializing)]
  parent: NodeWeakRef,
}

impl StyleRuleNode {
  pub fn new(txt: String, loc: Option<Loc>, parent: NodeWeakRef) -> HandleResult<Self> {
    let obj = Self {
      content: txt,
      loc,
      parent,
    };
    HandleResult::Success(obj)
  }
}
