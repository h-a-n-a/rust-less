#[cfg(test)]
mod tests {
  use std::borrow::Borrow;
  use std::cell::RefCell;
  use std::ops::Deref;
  use std::rc::Rc;
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
  
  #[derive(Clone)]
  struct Student {
    pub name: String,
    pub classmate: Option<Rc<RefCell<Student>>>,
  }
  
  impl Student {
    fn new() -> Student {
      Student {
        name: "1".to_string(),
        classmate: None,
      }
    }
    
    fn new_classmate(mut self) -> Student {
      self.name = "abc".to_string();
      let classmate = Rc::new(RefCell::new(self));
      Student {
        name: "2".to_string(),
        classmate: Some(classmate),
      }
    }
  }
  
  #[test]
  fn test_context_fn() {
    let a = Student::new();
    let b = a.new_classmate();
    println!(".......{}", b.classmate.unwrap().deref().borrow().name);
    println!(".......");
  }
}
