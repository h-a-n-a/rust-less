#[cfg(test)]
mod tests {
  use std::cell::RefCell;
  use std::ops::{Deref, DerefMut};
  use std::rc::Rc;
  use std::time::Duration;
  use tokio::time::sleep;
  use tokio::sync::Mutex;
  use crate::extend::string::StringExtend;
  use crate::extend::vec_str::VecStrExtend;
  use async_recursion::async_recursion;
  
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
    // let _t = &strore[1..1];
    // println!("......");
  }
  
  #[test]
  fn test_slice() {
    let a = "1233284920348aljdfalkdfjalkfdj023180";
    let mut i = 0;
    let target = "332";
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
  
  async fn add(num: Rc<RefCell<i32>>, flag: String) {
    let p = flag.parse::<i32>().unwrap();
    if p % 2 == 0 {
      sleep(Duration::from_secs(2)).await;
    }
    let mut i = 0;
    while i < 3 {
      *num.deref().borrow_mut().deref_mut() += 1;
      i += 1;
      let c = *num.deref().borrow().deref();
      println!("flag is {} num is {}", flag, c);
    }
  }
  
  #[test]
  fn test_future() {
    let num = Rc::new(RefCell::new(1));
    let rt = tokio::runtime::Runtime::new().unwrap();
    let d = rt.block_on(async {
      let exec_times: i32 = 20;
      let mut task_list = vec![];
      let mut index = 0;
      while index < exec_times {
        let task = add(num.clone(), index.to_string());
        task_list.push(task);
        index += 1;
      }
      futures::future::join_all(task_list).await;
      num
    });
    let c = *d.deref().borrow().deref();
    println!("{}", c);
    println!("........");
  }
  
  #[test]
  fn test_tokio() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
      sleep(Duration::from_secs(2)).await;
      println!("100 ms have elapsed");
    });
  }
  
  async fn add_mutex(num: &Mutex<i32>, flag: String) {
    let p = flag.parse::<i32>().unwrap();
    if p % 2 == 0 {
      sleep(Duration::from_secs(2)).await;
    }
    let mut i = 0;
    while i < 3 {
      let mut val = num.lock().await;
      *val += 1;
      i += 1;
      println!("flag is {} num is {}", flag, val);
    }
  }
  
  #[test]
  fn test_tokio_mutex() {
    let num = Mutex::new(1);
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
      let exec_times: i32 = 5;
      let mut task_list = vec![];
      let mut index = 0;
      while index < exec_times {
        let task = add_mutex(&num, index.to_string());
        task_list.push(task);
        index += 1;
      }
      futures::future::join_all(task_list).await;
    });
    let c = rt.block_on(async {
      num.lock().await.clone().to_string()
    });
    println!("{}", c);
    println!("........");
  }
  
  #[test]
  fn test_rc() {
    let a = Rc::new("123");
    println!("0->{}", Rc::strong_count(&a));
    let mut list = vec![];
    list.push(a.clone());
    println!("1->{}", Rc::strong_count(&a));
    {
      let mut list = vec![];
      list.push(a.clone());
      println!("2->{}", Rc::strong_count(&a));
    }
    list.remove(0);
    println!("3->{}", Rc::strong_count(&a));
  }
  
  #[test]
  fn test_display() {
    let str = "我是谁".to_string();
    let test = (
      str.charAt(None).unwrap(),
      str.charAt(Some(0)).unwrap(),
      str.charAt(Some(-1)).unwrap()
    );
    println!("{:#?}", test);
    assert_eq!(test.0.as_str(), "我");
    assert_eq!(test.1.as_str(), "我");
    assert_eq!(test.2.as_str(), "");
  }
  
  #[async_recursion]
  async fn ort_add(num: i32) -> i32 {
    println!("......{}", num);
    match num {
      0 => panic!("zero is not a valid argument to fib()!"),
      1 | 2 => 1,
      3 => 2,
      _ => {
        let a = ort_add(num - 1).await;
        let b = ort_add(num - 2).await;
        a + b
      }
    }
  }
  
  #[test]
  fn test_recyl() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async {
      ort_add(10).await
    });
    println!("{}", res);
    println!(".............")
  }
}
