#[cfg(test)]
mod tests {
  use crate::extend::string::StringExtend;
  use crate::extend::vec_str::VecStrExtend;

  #[test]
  fn test_str() {
    let strore = "123456";
    let _a = &strore[0..2];
    let _b = &strore[1..3];
    let index_1 = strore.to_string().indexOf("23", Some(2));
    let index_2 = strore.to_string().indexOf("23", Some(1));
    let index_3 = strore.to_string().indexOf("23", None);
    assert_eq!(index_1, -1);
    assert_eq!(index_2, 1);
    assert_eq!(index_3, 1);
    let t = &strore[1..1];
    // println!("......");
  }

  #[test]
  fn test_slice() {
    let a = "1233284920348aljdfalkdfjalkfdj023180";
    let mut i = 0;
    let mut target = "332";
    let mut copy: String = "".to_string();
    loop {
      if i < a.len() {
        let word = a.to_string().tocharlist().try_getword(i, 3).unwrap();
        // println!("word is {} ...", word);
        if i == 2 {
          copy = word.clone();
        }
      } else {
        break;
      }
      i += 1;
    }
    assert_eq!(copy.as_str(), target);
  }
}
