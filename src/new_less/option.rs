use crate::new_less::fileinfo::FileInfo;
use crate::new_less::parse::RuleNode;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseOption {
  pub include_path: Option<Vec<String>>,
  pub sourcemap: bool,
  pub tabspaces: usize,
}

impl Default for ParseOption {
  fn default() -> Self {
    ParseOption {
      include_path: None,
      sourcemap: true,
      tabspaces: 2,
    }
  }
}

pub trait OptionExtend {
  fn get_options(&self) -> ParseOption;
}

impl OptionExtend for FileInfo {
  fn get_options(&self) -> ParseOption {
    self.option.clone()
  }
}

impl OptionExtend for RuleNode {
  fn get_options(&self) -> ParseOption {
    match self.file_info.clone() {
      None => Default::default(),
      Some(obj) => obj.upgrade().unwrap().deref().borrow().option.clone(),
    }
  }
}
