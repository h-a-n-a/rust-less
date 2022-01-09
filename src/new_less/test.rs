#[cfg(test)]
mod tests {
  use crate::extend::time::wastetime;
  use crate::new_less::file::*;
  use crate::new_less::fileinfo::*;
  use crate::new_less::loc::LocMap;

  #[test]
  fn test_less() {
    let start_record = wastetime("test_less");
    // 处理过程
    let filepath = path_resolve("assets/demo.less");
    let info = FileInfo::create_disklocation(filepath, Default::default()).unwrap();
    start_record();
    // println!("{:#?}", info);
    println!("........");
  }

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
    assert_eq!(x.char, ";".to_string());
  }

  #[test]
  fn test_rel_path() {
    let a = "../test/a.txt".to_string();
    let b = "./test/a.txt".to_string();
    let c = "/test/a.txt".to_string();
    assert_eq!(FileManger::is_relative_path(&a), true);
    assert_eq!(FileManger::is_relative_path(&b), true);
    assert_eq!(FileManger::is_relative_path(&c), true);
  }
}