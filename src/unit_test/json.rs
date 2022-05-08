use std::collections::HashMap;
use serde_json::Value;
use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;
use crate::new_less::filenode::FileNode;

#[test]
fn test_less_json_deserializer() {
  // 处理过程
  let filepath = path_resolve("assets/demo.less");
  let app = Application::default();
  let info = app.parse(filepath.as_str()).unwrap();
  let json = serde_json::to_string_pretty(&info).unwrap();
  let root: HashMap<String, Value> = serde_json::from_str(&json).unwrap();
  let map = root.get("info").unwrap().as_object().unwrap();
  let res = FileNode::deserializer(map, app.context.borrow().weak_ref.as_ref().unwrap().upgrade().unwrap().clone()).unwrap();
  let back_json = serde_json::to_string_pretty(&res).unwrap();
  assert_eq!(json, back_json);
}
