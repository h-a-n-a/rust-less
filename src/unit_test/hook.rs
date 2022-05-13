use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;
use std::sync::Arc;

#[test]
fn test_less_import_hook() {
  let filepath = path_resolve("assets/demo.less");
  let app = Application::default();
  app.context.lock().unwrap().option.hooks.import_alias = Some(Arc::new(|filepath, importpath| {
    println!("{},{}", filepath, importpath);
    Ok(importpath)
  }));
  let res = app.render(filepath.as_str()).unwrap();
  println!("{}", res);
}

#[test]
fn test_less_content_hook() {
  // let filepath = path_resolve("assets/demo.less");
  let rt = tokio::runtime::Runtime::new().unwrap();
  rt.block_on(async {
    let filepath = path_resolve(
      "/Users/zhushijie/Desktop/github/rspack/examples/arco-pro/src/style/global.less",
    );
    let app = Application::default();
    let res = app.render_into_hashmap(filepath.as_str()).unwrap();
    println!("rust_res: -> \n {:#?}", res);
  });
}
