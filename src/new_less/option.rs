use crate::new_less::fileinfo::FileInfo;
use crate::new_less::hooks::ParseHooks;
use crate::new_less::parse::RuleNode;
use derivative::Derivative;
use std::ops::Deref;

#[derive(Derivative)]
#[derivative(Debug, PartialEq)]
#[derive(Clone)]
pub struct ParseOption {
  pub include_path: Vec<String>,
  pub sourcemap: bool,
  pub tabspaces: usize,
  #[derivative(Debug = "ignore", PartialEq = "ignore")]
  pub hooks: ParseHooks,
}

impl Default for ParseOption {
  fn default() -> Self {
    ParseOption {
      include_path: vec![],
      sourcemap: true,
      tabspaces: 2,
      hooks: Default::default(),
    }
  }
}

pub trait OptionExtend {
  fn get_options(&self) -> ParseOption;
}

impl OptionExtend for FileInfo {
  fn get_options(&self) -> ParseOption {
    self.context.deref().borrow().option.clone()
  }
}

impl OptionExtend for RuleNode {
  fn get_options(&self) -> ParseOption {
    self.context.deref().borrow().option.clone()
  }
}
