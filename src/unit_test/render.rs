use crate::extend::time::wastetime;
use crate::new_less::context::Context;
use crate::new_less::file::path_resolve;

#[test]
fn test_less_render() {
  let record = wastetime("test_less_render");
  let filepath = path_resolve("assets/demo.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  context.render(filepath).unwrap();
  record();
}
