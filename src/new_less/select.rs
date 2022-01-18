use std::convert::Infallible;
use crate::extend::string::StringExtend;
use crate::new_less::token::{Token, TokenComina, TokenSelect};

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
      origin_txt: txt.trim().to_string(),
      rule: vec![],
    };
    obj
  }
  
  pub fn value(&self) -> String {
    self.origin_txt.clone()
  }
  
  
  fn analysis(&self) {
    let charlist = self.origin_txt.tocharlist();
    
    let mut index = 0;
    let mut token_vec = vec![];
    let mut templist = vec![];
    
    let mut prev_token: Option<String> = None;
    let mut current_token: Option<String> = None;
    
    while index < charlist.len() {
      let char = charlist.get(index).unwrap().to_string();
      let prevchar = charlist.get(index - 1).unwrap_or(&"".to_string()).to_string();
      if Token::is_token(&char) {
        // 初始化的时候赋值
        if prev_token.is_none() {
          prev_token = Some(char.clone());
        } else {
          prev_token = Some(current_token.as_ref().unwrap().clone());
        }
        // skip  空格
        if Token::is_space_token(&prevchar) && Token::is_space_token(&char) {
          index += 1;
          continue;
        }
        current_token = Some(char.clone());
        let ct = current_token.as_ref().unwrap().clone().as_str();
        match TokenSelect::try_from(ct) {
          Ok(token) => {
            match token {
              TokenSelect::ClassToken => {}
              TokenSelect::IdToken => {}
              TokenSelect::WildCard => {}
              TokenSelect::AttrBegin => {}
              TokenSelect::AttrEnd => {}
              _ => {}
            }
          }
          Err(_) => {}
        }
        
        match TokenComina::try_from(ct) {
          Ok(token) => {
            match token {
              TokenComina::Comma => {}
              TokenComina::Space => {}
              TokenComina::NewLineOs => {}
              TokenComina::NewLineWindos => {}
              TokenComina::ExtendChar => {}
              TokenComina::ColumnChar => {}
              TokenComina::BrotherNextChar => {}
              TokenComina::BrotherMatchChar => {}
            }
          }
          Err(_) => {}
        }
        let temp_word = templist.join("");
        
        if !temp_word.is_empty() {
          token_vec.push(temp_word);
        }
        templist.clear();
      }
      
      templist.push(char.clone());
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