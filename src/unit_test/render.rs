use crate::extend::time::wastetime;
use crate::new_less::context::Context;
use crate::new_less::file::path_resolve;
use crate::new_less::option::ParseOption;

#[test]
fn test_less_render() {
  let record = wastetime("test_less_render");
  let filepath = path_resolve("assets/var.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let res = context.render(filepath).unwrap();
  println!("{}", res);
  record();
}

#[test]
fn test_less_render_without_sourcemap() {
  let record = wastetime("test_less_render");
  let filepath = path_resolve("assets/var.less");
  let context = Context::new(ParseOption {
    include_path: vec![],
    sourcemap: false,
    tabspaces: 2,
    hooks: Default::default(),
  }, Some(filepath.clone())).unwrap();
  let res = context.render(filepath).unwrap();
  println!("{}", res);
  record();
}

#[test]
fn test_less_arco_pro_render() {
  let record = wastetime("test_less_render");
  let mut index = 1;
  let mut error_file = vec![];
  while index < 39 {
    let filepath = path_resolve(format!("assets/arco-pro/{}.less", index).as_str());
    let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
    let res = match context.render(filepath.clone()) {
      Ok(res) => {
        res
      }
      Err(msg) => {
        error_file.push(filepath);
        msg
      }
    };
    println!("{}", res);
    index += 1;
  }
  println!("{:#?}", error_file);
  record();
}
