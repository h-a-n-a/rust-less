use crate::new_less::fileinfo::FileInfo;
use crate::new_less::parse::RuleNode;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseOption {
  pub include_path: Option<Vec<String>>,
  pub sourcemap: bool,
}

impl Default for ParseOption {
  fn default() -> Self {
    ParseOption {
      include_path: None,
      sourcemap: true,
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
    self.option.clone()
  }
}
