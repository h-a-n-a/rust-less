#[allow(non_snake_case)]
pub trait StringExtend {
  fn charCodeAt(&self, index: usize) -> Option<u32>;
  fn charAt(&self, index: usize) -> Option<String>;
  fn indexOf(&self, findchar: &str, fromindex: Option<usize>) -> i32;
  fn slice(&self, fromindex: i32) -> String;
  fn substr(&self, fromindex: i32, length: Option<i32>) -> String;
  fn tocharlist(&self) -> Vec<String>;
}

#[allow(non_snake_case)]
impl StringExtend for String {
  fn charCodeAt(&self, index: usize) -> Option<u32> {
    let charlist: Vec<char> = self.chars().collect::<Vec<char>>();
    match charlist.get(index) {
      Some(val) => Some(*val as u32),
      None => None
    }
  }
  fn charAt(&self, index: usize) -> Option<String> {
    let charlist: Vec<char> = self.chars().collect::<Vec<char>>();
    match charlist.get(index) {
      Some(val) => Some(val.to_string()),
      None => None
    }
  }
  
  fn indexOf(&self, findchar: &str, fromindex: Option<usize>) -> i32 {
    let list = self.chars().into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let len = list.len();
    let mut res = -1;
    let start = fromindex.unwrap_or(0);
    if start > len - 1 {
      return res;
    }
    let mut index = start;
    let step = findchar.len();
    loop {
      let search_start = index;
      let mut search_end = index + step;
      if search_end > len - 1 {
        search_end = len - 1;
      }
      if index < len {
        let mut cc = "".to_string();
        list[search_start..search_end].iter().for_each(|x| {
          cc += x.as_str();
        });
        if cc.as_str() == findchar {
          res = index as i32;
          break;
        }
      } else {
        break;
      }
      index += 1;
    }
    res
  }
  
  fn slice(&self, fromindex: i32) -> String {
    let len = self.len() as i32;
    if fromindex > len {
      return "".to_string();
    }
    return if fromindex >= 0 {
      self.clone().as_str()[fromindex as usize..].to_string()
    } else {
      let mut rev_start = len + fromindex;
      if rev_start < 0 {
        rev_start = 0;
      }
      self.clone().as_str()[rev_start as usize..].to_string()
    };
  }
  
  fn substr(&self, fromindex: i32, length: Option<i32>) -> String {
    let len = self.len() as i32;
    if length.is_some() && length.unwrap() <= 0 {
      return "".to_string();
    }
    if fromindex >= len {
      return "".to_string();
    }
    return if fromindex < 0 {
      self.slice(fromindex)
    } else {
      let start = fromindex as usize;
      let end: usize;
      if length.is_none() {
        end = len as usize;
      } else {
        end = (fromindex + length.unwrap()) as usize;
      }
      self.clone().as_str()[start..end].to_string()
    };
  }
  
  fn tocharlist(&self) -> Vec<String> {
    self.chars().map(|x| x.to_string()).collect::<Vec<String>>()
  }
}