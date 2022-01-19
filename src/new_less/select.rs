use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;
use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::token::lib::Token;
use crate::new_less::token::select::{TokenCombina, TokenSelect};

///
/// 选择器范式
///
#[derive(Debug)]
pub enum SelectParadigm {
  // 选择器
  TokenSelect(String),
  
  // 选择链接器
  TokenComina(String),
  
  // 选择元素
  TokenTag(String),
  
  // 其他token
  TokenOther(String),
  
}

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
  pub fn new(txt: String) -> Result<Selector, String> {
    let obj = Selector {
      origin_txt: txt.trim().to_string(),
      rule: vec![],
    };
    match obj.parse() {
      Ok(()) => {
        Ok(obj)
      }
      Err(msg) => {
        Err(msg)
      }
    }
  }
  
  pub fn value(&self) -> String {
    self.origin_txt.clone()
  }
  
  
  fn parse(&self) -> Result<(), String> {
    let charlist = self.origin_txt.tocharlist();
    
    let mut index = 0;
    let mut token_vec: Vec<SelectParadigm> = vec![];
    let mut templist = vec![];
    
    let mut current_token: Option<String> = None;
    
    while index < charlist.len() {
      let char = charlist.get(index).unwrap().to_string();
      let nextchar = charlist.get(index + 1).unwrap_or(&"".to_string()).to_string();
      let mut include_attr = false;
      if Token::is_token(&char) && !include_attr {
        // 处理之前的 缓冲区的值
        if index > 0 && current_token.is_none() {
          token_vec.push(SelectParadigm::TokenTag(templist.poly()));
        } else if current_token.is_some() {
          if TokenSelect::is(&current_token.as_ref().unwrap()) {
            token_vec.push(SelectParadigm::TokenSelect(templist.poly()));
          }
        }
        templist.clear();
        // 替换现有的 token
        current_token = Some(char.clone());
        // skip  空格
        if Token::is_space_token(&char) && Token::is_space_token(&nextchar) {
          index += 1;
          continue;
        }
        // 处理后续值的情况
        
        if TokenSelect::is(&current_token.as_ref().unwrap()) {
          templist.push(char.clone());
          index += 1;
          continue;
        } else if TokenCombina::is(&current_token.as_ref().unwrap()) {
          if &char == "|" {
            if nextchar == "|" {
              index += 2;
              current_token = Some("||".to_string());
            } else {
              return Err(r#" "|" is not allow exist!"#.to_string());
            }
          } else {
            index += 1;
          }
          token_vec.push(SelectParadigm::TokenComina(current_token.as_ref().unwrap().clone()));
          continue;
        } else {
          token_vec.push(SelectParadigm::TokenOther(current_token.as_ref().unwrap().clone()));
          index += 1;
          continue;
        }
      } else {
        templist.push(char.clone());
      }
      
      if index == charlist.len() - 1 {
        if current_token.is_none() || TokenCombina::is(&current_token.as_ref().unwrap()) {
          token_vec.push(SelectParadigm::TokenTag(templist.poly()));
        } else {
          if TokenSelect::is(&current_token.as_ref().unwrap()) {
            token_vec.push(SelectParadigm::TokenSelect(templist.poly()));
          }
        }
        templist.clear();
      }
      index += 1;
    }
    
    println!("{:#?}", token_vec);
    
    Ok(())
  }
}