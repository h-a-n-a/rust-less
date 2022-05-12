use std::sync::Arc;
use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;
use crate::new_less::hooks::ParseHooks;

#[test]
fn test_less_render() {
  let filepath = path_resolve("assets/demo.less");
  let app = Application::default();
  app.context.lock().unwrap().option.hooks = ParseHooks {
    import_alias: Some(Arc::new(|filepath, importpath| {
      println!("{},{}", filepath, importpath);
      Ok(importpath)
    }))
  };
  let res = app.render(filepath.as_str()).unwrap();
  println!("{}", res);
}