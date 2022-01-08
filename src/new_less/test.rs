#[cfg(test)]
mod tests {
  use std::time::Instant;
  use crate::new_less::file::*;
  use crate::new_less::fileinfo::FileManger;
  use crate::new_less::loc::LocMap;
  use crate::new_less::origin_parse::*;

  #[test]
  fn test_less() {
    let now = Instant::now();
    let content = readfile(path_resolve("assets/demo.less")).unwrap();
    /// 转化 Less 文件中 最原始的 Block 片段
    let blocks = parse_origin_block(content).unwrap();
    let end = now.elapsed();
    println!("{:#?}", blocks);
    let a = end.as_micros() as f32;
    println!("Running slow_function() took {} ms", a * 0.001);
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
    assert_eq!(FileManger::is_relative_path(&a) , true);
    assert_eq!(FileManger::is_relative_path(&b), true);
    assert_eq!(FileManger::is_relative_path(&c), true);
  }
}