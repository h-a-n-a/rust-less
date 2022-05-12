use std::sync::Arc;
use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;

#[test]
fn test_less_import_hook() {
  let filepath = path_resolve("assets/demo.less");
  let app = Application::default();
  app.context.lock().unwrap().option.hooks.import_alias =
    Some(Arc::new(|filepath, importpath| {
      println!("{},{}", filepath, importpath);
      Ok(importpath)
    }));
  let res = app.render(filepath.as_str()).unwrap();
  println!("{}", res);
}

#[test]
fn test_less_content_hook() {
  let filepath = path_resolve("assets/demo.less");
  let app = Application::default();
  let res = app.render(filepath.as_str()).unwrap();
  println!("rust_res: -> \n {}", res);
}