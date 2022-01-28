use crate::new_less::loc::Loc;
use crate::new_less::node::HandleResult;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StyleRuleNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Option<Loc>,
}

impl StyleRuleNode {
  pub fn new(txt: String, loc: Option<Loc>) -> HandleResult<Self> {
    let obj = Self { content: txt, loc };
    HandleResult::Success(obj)
  }
}
