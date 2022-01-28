use crate::new_less::loc::Loc;
use crate::new_less::node::HandleResult;
use serde::Serialize;
use crate::new_less::option::ParseOption;

#[derive(Debug, Clone, Serialize)]
pub struct StyleRuleNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Option<Loc>,

  // 内部调用方式时 需要拿到对应的 转化配置
  #[serde(skip_serializing)]
  option: ParseOption,
}

impl StyleRuleNode {
  pub fn new(txt: String, loc: Option<Loc>, option: ParseOption) -> HandleResult<Self> {
    let obj = Self { content: txt, loc, option };
    HandleResult::Success(obj)
  }
}
