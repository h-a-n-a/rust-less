use crate::new_less::file::{path_resolve, readfile};
use crate::new_less::loc::LocMap;

///
/// 测试字典方法
///
#[test]
fn test_loc() {
    let content = readfile(path_resolve("assets/demo.less")).unwrap();
    let obj = LocMap::new(content);
    let c = obj.get(0).unwrap();
    let x = obj.getloc(17, 25).unwrap();
    assert_eq!(c.char, "@".to_string());
    assert_eq!(x.char, "e".to_string());
}
