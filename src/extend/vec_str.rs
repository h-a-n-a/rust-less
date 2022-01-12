pub trait VecStrExtend {
  fn try_getword(&self, index: usize, wordlength: usize) -> Result<String, String>;
}

impl VecStrExtend for Vec<String> {
  fn try_getword(&self, index: usize, wordlength: usize) -> Result<String, String> {
    if index < self.len() {
      let start = index;
      let mut end = index + wordlength;
      if end > self.len() {
        end = self.len();
      }
      Ok(self[start..end].join(""))
    } else {
      Err("find index is over vec range!".to_string())
    }
  }
}

impl VecStrExtend for [String] {
  fn try_getword(&self, index: usize, wordlength: usize) -> Result<String, String> {
    if index < self.len() {
      let start = index;
      let mut end = index + wordlength;
      if end > self.len() {
        end = self.len();
      }
      Ok(self[start..end].join(""))
    } else {
      Err("find index is over vec range!".to_string())
    }
  }
}