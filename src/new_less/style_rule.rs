use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::FileWeakRef;
use crate::new_less::loc::Loc;
use crate::new_less::node::{HandleResult, NodeWeakRef};
use crate::new_less::option::ParseOption;
use derivative::Derivative;
use serde::Serialize;

#[derive(Derivative, Serialize, Clone)]
#[derivative(Debug)]
pub struct StyleRuleNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Option<Loc>,

  // 文件信息
  #[serde(skip_serializing)]
  pub fileinfo: FileWeakRef,

  // 上下文
  #[derivative(Debug = "ignore")]
  #[serde(skip_serializing)]
  pub context: ParseContext,
}

impl StyleRuleNode {
  pub fn new(
    txt: String,
    loc: Option<Loc>,
    _parent: NodeWeakRef,
    fileinfo: FileWeakRef,
    context: ParseContext,
  ) -> HandleResult<Self> {
    let obj = Self {
      content: txt,
      loc,
      fileinfo,
      context,
    };
    HandleResult::Success(obj)
  }

  ///
  /// 获取选项
  ///
  pub fn get_options(&self) -> ParseOption {
    self.context.borrow().option.clone()
  }
}
