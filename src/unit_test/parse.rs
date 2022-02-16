use crate::extend::time::wastetime;
use crate::new_less::context::Context;
use crate::new_less::file::path_resolve;

#[test]
fn test_less_parse() {
  let record = wastetime("test_less");
  // 处理过程
  let filepath = path_resolve("assets/demo.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let info = context.parse(filepath).unwrap();
  record();
  let json = serde_json::to_string_pretty(&info.borrow().tojson()).unwrap();
  println!("{}", json);
}

#[test]
fn test_less() {
  let record = wastetime("test_less");
  // 处理过程
  let filepath = path_resolve("assets/demo.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let info = context.render(filepath).unwrap();
  record();
  println!("{}", info);
  println!(".....")
}
