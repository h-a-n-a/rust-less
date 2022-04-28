use crate::extend::string::StringExtend;
use crate::new_less::file::{path_resolve, readfile};
use crate::new_less::loc::LocMap;

///
/// 测试字典方法
///
#[test]
fn test_loc() {
  let content = readfile(path_resolve("assets/loc.less").as_str()).unwrap();
  let obj = LocMap::new(&content.tocharlist());
  let c = obj.get(&0).unwrap();
  let x = obj.getloc(4, 10).unwrap();
  assert_eq!(c.char, '@');
  assert_eq!(x.char, '@');
}
