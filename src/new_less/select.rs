use std::ops::Deref;
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
  SelectWrap(String),
  
  // 选择链接器
  CominaWrap(String),
  
  // 选择元素
  TagWrap(String),
  
  // 其他token
  OtherWrap(String),
  
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
  
  ///
  /// 创建闭包 填词
  ///
  fn create_select_token_task(init: String) -> Box<dyn FnMut(Option<String>) -> String> {
    let mut content = init;
    Box::new(move |txt: Option<String>| -> String {
      content = content.clone() + &txt.unwrap_or("".to_string());
      content.clone()
    })
  }
  
  fn parse(&self) -> Result<(), String> {
    let charlist = self.origin_txt.tocharlist();
    let mut index = 0;
    let mut token_vec: Vec<SelectParadigm> = vec![];
    let mut current_token: Option<String> = None;
    let mut paradigm_vec: Vec<SelectParadigm> = vec![];
    let mut include_attr = false;
    let mut write_task: Option<Box<dyn FnMut(Option<String>) -> String>> = None;
    
    while index < charlist.len() {
      let char = charlist.get(index).unwrap().to_string();
      let nextchar = charlist.get(index + 1).unwrap_or(&"".to_string()).to_string();
      // 有任务则继续填词
      if write_task.is_some() && Token::is_token(&char) {
        let task = write_task.unwrap().deref();
      }
      if index == 0 {
        if Token::is_token(&char) {
          // 是符号
          if TokenSelect::is(&char) {} else if TokenCombina::is(&char) {} else {}
        } else {
          // Tag启动
          write_task = Some(Self::create_select_token_task(char.clone()));
        }
      }
      if Token::is_token(&char) && !include_attr {} else {}
      index += 1;
    }
    
    println!("{:#?}", token_vec);
    
    Ok(())
  }
}