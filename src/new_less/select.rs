use crate::extend::string::StringExtend;

#[derive(Debug, Clone)]
pub struct Selector {
  origin_txt: String,
  rule: Vec<SelectorRule>,
}

#[derive(Debug, Clone)]
pub struct SelectorRule {
  origin_txt: String,
}

impl Selector {
  ///
  /// 初始化方法
  ///
  pub fn new(txt: String) -> Selector {
    let obj = Selector {
      origin_txt: txt,
      rule: vec![],
    };
    obj.analysis();
    obj
  }
  
  pub fn value(&self) -> String {
    self.origin_txt.clone()
  }
  
  pub fn get_token() -> Vec<String> {
    vec![".", "#", "~", " ", "\n", "\r", "|", ":", "[", "]", "@", "/", "+", "*", "-", "_", "(", ")", ";", "'", r#"""#]
      .into_iter()
      .map(|x| x.to_string()).collect()
  }
  
  pub fn is_token(char: &str) -> bool {
    match Selector::get_token().into_iter().find(|x| { x == char }) {
      None => { false }
      Some(_) => { true }
    }
  }
  
  
  fn analysis(&self) {
    let charlist = self.origin_txt.tocharlist();
    let mut index = 0;
    let mut token_vec = vec![];
    let mut templist = vec![];
    while index < charlist.len() {
      let char = charlist.get(index).unwrap().to_string();
      if Selector::is_token(char.as_str()) {
        let temp_word = templist.join("");
        if !temp_word.is_empty() {
          token_vec.push(temp_word);
        }
        templist.clear();
        token_vec.push(char);
      } else {
        templist.push(char.clone());
      }
      index += 1;
    }
    
    let temp_word = templist.join("");
    if !temp_word.is_empty() {
      token_vec.push(temp_word);
      templist.clear();
    }
    
    
    println!("{}", token_vec.join(","));
  }
}