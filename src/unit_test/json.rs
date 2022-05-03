use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;

#[test]
fn test_less_parse() {
  // 处理过程
  let filepath = path_resolve("assets/demo.less");
  let app = Application::default();
  let info = app.parse(filepath).unwrap();
  let json = serde_json::to_string_pretty(&info).unwrap();
  println!("{}", json);
}
