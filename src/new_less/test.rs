#[cfg(test)]
mod tests {
  use crate::new_less::file::*;
  use crate::new_less::origin_parse::*;
  
  #[test]
  fn test_less() {
    let content = readfile(path_resolve("assets/demo.less")).unwrap();
    /// 转化 Less 文件中 最原始的 Block 片段
    let blocks = parse_origin_block(content).unwrap();
    println!("{:#?}", blocks);
    println!("........");
  }
}