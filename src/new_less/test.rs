#[cfg(test)]
mod tests {
  use crate::new_less::file::*;
  use crate::new_less::loc::LocMap;
  use crate::new_less::origin_parse::*;
  
  #[test]
  fn test_less() {
    let content = readfile(path_resolve("assets/demo.less")).unwrap();
    /// 转化 Less 文件中 最原始的 Block 片段
    let blocks = parse_origin_block(content).unwrap();
    println!("{:#?}", blocks);
    println!("........");
  }
  
  #[test]
  fn test_loc() {
    let content = readfile(path_resolve("assets/demo.less")).unwrap();
    let obj = LocMap::new(content);
    let c = obj.get(0).unwrap();
    let x = obj.getloc(17, 25).unwrap();
    assert_eq!(c.char, "@".to_string());
    assert_eq!(x.char, ";".to_string());
  }
}