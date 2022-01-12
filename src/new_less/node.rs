use std::cell::RefCell;
use std::rc::Rc;
use crate::new_less::comment::CommentNode;
use crate::new_less::parse::{RuleNode, RuleNodeJson};
use crate::new_less::var::VarNode;
use serde::{Serialize};

#[derive(Debug, Clone)]
pub enum StyleNode {
  Comment(CommentNode),
  Var(VarNode),
  Rule(Rc<RefCell<RuleNode>>),
}


#[derive(Debug, Clone, Serialize)]
pub enum StyleNodeJson {
  Comment(CommentNode),
  Var(VarNode),
  Rule(RuleNodeJson),
}

