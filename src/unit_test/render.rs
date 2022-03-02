use crate::extend::time::wastetime;
use crate::new_less::context::Context;
use crate::new_less::file::path_resolve;

#[test]
fn test_less_render() {
  let record = wastetime("test_less_render");
  let filepath = path_resolve("assets/demo.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let res = context.render(filepath).unwrap();
  println!("{}", res);
  record();
}

#[test]
fn test_less_arco_pro_render() {
  let record = wastetime("test_less_render");
  let index = 13;
  let filepath = path_resolve(format!("assets/arco-pro/{}.less", index).as_str());
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let res = context.render(filepath).unwrap();
  println!("{}", res);
  record();
}
