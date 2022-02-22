use crate::new_less::comment::CommentNode;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::{FileRef, FileWeakRef};
use crate::new_less::import::ImportNode;
use crate::new_less::loc::Loc;
use crate::new_less::parse::{RuleNode, RuleNodeJson};
use crate::new_less::style_rule::StyleRuleNode;
use crate::new_less::var_node::VarNode;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type NodeWeakRef = Option<Weak<RefCell<RuleNode>>>;
pub type NodeRef = Rc<RefCell<RuleNode>>;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum StyleNode {
  Comment(CommentNode),
  Var(VarRuleNode),
  Rule(NodeRef),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
pub enum StyleNodeJson {
  Comment(CommentNode),
  Var(VarRuleNode),
  Rule(RuleNodeJson),
}

///
/// 处理类型
///
pub enum HandleResult<T> {
  /// 匹配成功 且 处理成功
  Success(T),

  /// 匹配成功 且 处理失败
  Fail(String),

  /// 匹配失败
  Swtich,
}

///
/// 变量内容
///
#[derive(Debug, Clone, Serialize)]
pub enum VarRuleNode {
  /// 引用
  Import(ImportNode),

  /// 变量声明
  Var(VarNode),

  /// 样式规则
  StyleRule(StyleRuleNode),
}

///
/// 联合 节点 声明
///
impl VarRuleNode {
  ///
  /// 初始化
  ///
  pub fn new(
    txt: String,
    loc: Option<Loc>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
    context: ParseContext,
    importfiles: &mut Vec<FileRef>,
  ) -> Result<Self, String> {
    // 处理 导入
    match ImportNode::new(
      txt.clone(),
      loc.clone(),
      parent.clone(),
      fileinfo.clone(),
      context.clone(),
      importfiles,
    ) {
      HandleResult::Success(obj) => return Ok(VarRuleNode::Import(obj)),
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    // 处理 变量声明
    match VarNode::new(
      txt.clone(),
      loc.clone(),
      parent.clone(),
      fileinfo.clone(),
      context.clone(),
    ) {
      HandleResult::Success(obj) => return Ok(VarRuleNode::Var(obj)),
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    // 处理 规则
    match StyleRuleNode::new(txt.clone(), loc, parent, fileinfo, context) {
      HandleResult::Success(obj) => return Ok(VarRuleNode::StyleRule(obj)),
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    Err(format!("nothing node match the txt -> {}", txt))
  }
}
