use crate::extend::time::wastetime;
use crate::new_less::file::path_resolve;
use crate::new_less::fileinfo::FileInfo;
use crate::new_less::option::ParseOption;
use std::ops::Deref;

#[test]
fn test_less() {
  let record = wastetime("test_less");
  // 处理过程
  let filepath = path_resolve("assets/demo.less");
  let info = FileInfo::create_disklocation(filepath, Default::default()).unwrap();
  record();
  let json = serde_json::to_string_pretty(&info.borrow().tojson()).unwrap();
  println!("{}", json);
}

