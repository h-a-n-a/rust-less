use crate::extend::string::StringExtend;
use serde::{Serialize};

///
/// 媒体查询
///
#[derive(Debug, Clone, Serialize)]
pub struct MediaQuery {
  pub origin_txt: String,
  charlist: Vec<String>,
}

impl MediaQuery {

  ///
  /// 初始化方法
  ///
  pub fn new(txt: String) -> Result<Self, String> {
    let _obj = Self {
      origin_txt: txt.clone(),
      charlist: txt.tocharlist(),
    };
    // Ok(obj)
    Err("".to_string())
  }

  pub fn value(&self) -> String {
    self.origin_txt.clone()
  }
}