use crate::new_less::context::{Context, ParseContext};
use crate::new_less::fileinfo::{FileInfo, FileRef};
use crate::new_less::option::ParseOption;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Application {
  context: ParseContext,
}

impl Application {
  pub fn new(option: ParseOption, application_fold: Option<String>) -> Result<Self, String> {
    let context = Context::new(option, application_fold)?;
    Ok(Application {
      context: Rc::new(RefCell::new(context)),
    })
  }

  ///
  /// 产生代码
  ///
  pub fn render(&self, filepath: String) -> Result<String, String> {
    FileInfo::create_disklocation(filepath, self.context.clone())
  }

  ///
  /// 解析代码
  ///
  pub fn parse(self, filepath: String) -> Result<FileRef, String> {
    FileInfo::create_disklocation_parse(filepath, self.context.clone())
  }

  ///
  /// 生成默认上下文
  ///
  pub fn default() -> Application {
    Self::new(Default::default(), None).unwrap()
  }
}
