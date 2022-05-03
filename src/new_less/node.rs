use crate::new_less::comment::CommentNode;
use crate::new_less::rule::RuleNode;
use crate::new_less::var::VarRuleNode;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type NodeWeakRef = Option<Weak<RefCell<RuleNode>>>;
pub type NodeRef = Rc<RefCell<RuleNode>>;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum StyleNode {
  Comment(CommentNode),
  Var(VarRuleNode),
  Rule(NodeRef),
}
