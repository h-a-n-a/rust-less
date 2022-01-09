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