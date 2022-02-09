use std::rc::Rc;

#[derive(Clone)]
pub struct ParseHooks {
  import_alias: Option<Rc<dyn Fn(String) -> String>>,
}

impl Default for ParseHooks {
  fn default() -> Self {
    ParseHooks { import_alias: None }
  }
}
