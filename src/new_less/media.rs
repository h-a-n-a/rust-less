use crate::extend::string::StringExtend;
use serde::{Serialize};
use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::scan::{ScanArg, ScanResult, traversal};
use crate::new_less::token::lib::Token;
use crate::new_less::token::media::{TokenMediaFeature, TokenMediaLogic, TokenMediaType, TokenMeidaAllow};


///
/// 媒体查询
///
#[derive(Debug, Clone, Serialize)]
pub struct MediaQuery {
  pub origin_txt: String,
  charlist: Vec<String>,
}

impl MediaQuery {
  ///
  /// 初始化方法
  ///
  pub fn new(txt: String) -> Result<Self, String> {
    let mut obj = Self {
      origin_txt: txt.clone(),
      charlist: txt.trim().to_string().tocharlist(),
    };
    match obj.parse() {
      Ok(_) => {
        Ok(obj)
      }
      Err(msg) => {
        Err(msg)
      }
    }
  }
  
  ///
  /// 打印错误信息
  ///
  pub fn errormsg(&self, index: &usize) -> Result<(), String> {
    let char = self.charlist.get(*index).unwrap().clone();
    Err(format!("select text {}, char {} is not allow,index is {}", self.origin_txt, char, index))
  }
  
  pub fn value(&self) -> String {
    self.origin_txt.clone()
  }
  
  ///
  /// 子转化 媒体功能 转化 key
  ///
  pub fn parse_media_feature_key(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = self.charlist.clone();
    match traversal(Some(start.clone()), charlist, &mut (|arg, charword| {
      let mut hasend = arg.hasend;
      let mut temp = arg.temp;
      let index = arg.index;
      let (_, char, next) = charword;
      if Token::is_token(&char) {
        if char == TokenMeidaAllow::Colon.tostr_value() {
          if TokenMediaFeature::is(&temp) {
            // 加冒号之前 先判断是否是有效 key
            hasend = true;
          } else {
            return Err(self.errormsg(&index).err().unwrap());
          }
        } else if Token::is_space_token(&char) {
          if Token::is_space_token(&next) {
            return Ok(ScanResult::Skip);
          } else {
            temp += &char;
          }
        } else if &char == "-" {
          temp += "-";
        } else {
          return Err(self.errormsg(&index).err().unwrap());
        }
      } else {
        temp += &char;
      }
      Ok(ScanResult::Arg(ScanArg {
        temp,
        index,
        hasend,
      }))
    })) {
      Ok(res) => {
        Ok(res)
      }
      Err(msg) => {
        Err(msg)
      }
    }
  }
  
  ///
  /// 子转化 媒体功能 转化 value
  ///
  pub fn parse_media_value(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = self.charlist.clone();
    match traversal(
      Some(start.clone()),
      charlist,
      &mut (|arg, charword| {
        let mut hasend = arg.hasend;
        let mut temp = arg.temp;
        let index = arg.index;
        let (_, char, next) = charword;
        if Token::is_token(&char) {
          if char == TokenMeidaAllow::RightBrackets.tostr_value() {
            hasend = true;
          } else if Token::is_space_token(&char) {
            if Token::is_space_token(&next) {
              return Ok(ScanResult::Skip);
            } else {
              temp += &char;
            }
          } else if &char == "-" {
            temp += "-";
          } else {
            return Err(self.errormsg(&index).err().unwrap());
          }
        } else {
          temp += &char;
        }
        Ok(ScanResult::Arg(ScanArg {
          temp,
          index,
          hasend,
        }))
      }),
    ) {
      Ok(res) => {
        Ok(res)
      }
      Err(msg) => {
        Err(msg)
      }
    }
  }
  
  
  ///
  /// 子转化 媒体功能
  ///
  pub fn parse_media_feature(&mut self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let mut index = *start + 1;
    let mut word_vec: Vec<String> = vec!["(".to_string()];
    
    // 分析key
    let (key, jump) = match self.parse_media_feature_key(&index.clone()) {
      Ok(res) => { res }
      Err(msg) => { return Err(msg); }
    };
    word_vec.push(key);
    word_vec.push(":".to_string());
    index = jump + 1;
    
    // 分析value
    let (value, jump) = match self.parse_media_value(&index.clone()) {
      Ok(res) => { res }
      Err(msg) => { return Err(msg); }
    };
    word_vec.push(value);
    word_vec.push(")".to_string());
    index = jump + 1;
    
    if index < charlist.len() {
      return Err(self.errormsg(&index).err().unwrap());
    }
    
    Ok((word_vec.poly(), index))
  }
  
  ///
  /// 转化代码
  ///
  pub fn parse(&mut self) -> Result<(), String> {
    let charlist = self.charlist.clone();
    let mut temp: String = "".to_string();
    
    if charlist.len() < 6 ||
      (charlist.len() == 6 && charlist[0..6].poly().as_str() != "@media") ||
      (charlist.len() > 6 && charlist[0..7].poly().as_str() != "@media ") {
      return Err("select_txt not match media query".to_string());
    }
    let mut word_vec = vec!["@media".to_string()];
    let mut index = 6;
    
    // 循环解析
    while index < charlist.len() {
      let char = charlist.get(index).unwrap().to_string();
      let nextchar = if index == charlist.len() - 1 {
        "".to_string()
      } else {
        charlist.get(index + 1).unwrap().to_string()
      };
      if Token::is_token(&char) {
        if Token::is_space_token(&char) {
          if !Token::is_space_token(&nextchar) {
            let word = temp.clone();
            if TokenMediaLogic::is(&word) || TokenMediaType::is(&word) || word.is_empty() {
              word_vec.push(temp.clone());
              temp = "".to_string();
              word_vec.push(" ".to_string());
            } else {
              return Err(self.errormsg(&index).err().unwrap());
            }
          } else {
            index += 1;
            continue;
          }
        } else if TokenMeidaAllow::is(&char) {
          match TokenMeidaAllow::try_from(char.as_str()).unwrap() {
            TokenMeidaAllow::LeftBrackets => {
              match self.parse_media_feature(&index) {
                Ok((word, jump)) => {
                  word_vec.push(word);
                  temp = "".to_string();
                  index = jump + 1;
                  continue;
                }
                Err(msg) => {
                  return Err(msg);
                }
              }
            }
            _ => {
              return Err(self.errormsg(&index).err().unwrap());
            }
          }
        } else {
          return Err(self.errormsg(&index).err().unwrap());
        }
      } else {
        temp += &char;
      }
      index += 1;
    }
    Ok(())
  }
}