use crate::extend::string::StringExtend;
use serde::{Serialize};
use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;
use crate::extend::vec_str::VecStrExtend;
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
  pub fn errormsg(&mut self, index: &usize) -> Result<(), String> {
    let char = self.charlist.get(*index).unwrap().clone();
    Err(format!("select text {}, char {} is not allow,index is {}", self.origin_txt, char, index))
  }
  
  pub fn value(&self) -> String {
    self.origin_txt.clone()
  }
  
  ///
  /// 子转化 媒体功能
  ///
  pub fn parse_media_feature(&mut self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let mut index = *start + 1;
    let mut temp: String = "".to_string();
    let mut hasend = false;
    let mut haskey: bool = false;
    let mut word_vec: Vec<String> = vec!["(".to_string()];
    
    while index < charlist.len() {
      let prevchar = if index == 0 {
        "".to_string()
      } else {
        charlist.get(index - 1).unwrap().to_string()
      };
      let char = charlist.get(index).unwrap().to_string();
      let nextchar = if index == charlist.len() - 1 {
        "".to_string()
      } else {
        charlist.get(index + 1).unwrap().to_string()
      };
      // 分析value
      // 分析key
      if Token::is_token(&char) {
        if !temp.is_empty() && &char != "-" {
          word_vec.push(temp.clone());
          temp = "".to_string();
        }
        if char == TokenMeidaAllow::RightBrackets.tostr_value() {
          // 右括号完结
          hasend = true;
          word_vec.push(")".to_string());
          break;
        } else if Token::is_space_token(&char) {
          // 检查空格
          if Token::is_space_token(&nextchar) {
            // 连续空格跳过
            index += 1;
            continue;
          } else {
            // 非连续记录
            word_vec.push(" ".to_string());
          }
        } else if &char == "-" {
          if !haskey || (haskey && prevchar == TokenMeidaAllow::Colon.tostr_value()) {
            temp += &char;
          } else {
            return Err(self.errormsg(&index).err().unwrap());
          }
        } else if char == TokenMeidaAllow::Colon.tostr_value() {
          if !haskey {
            let feature = match word_vec.last() {
              None => { "".to_string() }
              Some(val) => { val.clone() }
            };
            if !feature.is_empty() && TokenMediaFeature::is(&feature) {
              haskey = true;
            } else {
              return Err(self.errormsg(&index).err().unwrap());
            }
            word_vec.push(TokenMeidaAllow::Colon.tostr_value());
          } else {
            // 防止多次出现冒号
            return Err(self.errormsg(&index).err().unwrap());
          }
        } else {
          return Err(self.errormsg(&index).err().unwrap());
        }
      } else {
        temp += &char
      }
      index += 1;
    }
    if !hasend || !haskey {
      return Err(format!("select text {}, not found ')'", self.origin_txt));
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
      return Err(format!("select_txt not match media query"));
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