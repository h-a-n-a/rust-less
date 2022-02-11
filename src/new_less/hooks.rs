use std::rc::Rc;

#[derive(Clone, Default)]
pub struct ParseHooks {
  pub import_alias: Option<Rc<dyn Fn(String) -> String>>,
}
