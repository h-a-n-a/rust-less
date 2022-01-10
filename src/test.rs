#[cfg(test)]
mod tests {
  use crate::extend::string::StringExtend;
  
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
  
  
  #[test]
  fn test_str() {
    let strore = "123456";
    let _a = &strore[0..2];
    let _b = &strore[1..3];
    let index_1 = strore.to_string().indexOf("23", Some(2));
    let index_2 = strore.to_string().indexOf("23", Some(1));
    let index_3 = strore.to_string().indexOf("23", None);
    println!("......");
    assert_eq!(index_1, -1);
    assert_eq!(index_2, 1);
    assert_eq!(index_3, 1);
    let t = &strore[1..1];
    println!("......");
  }
  
}
