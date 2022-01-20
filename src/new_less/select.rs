use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;
use crate::extend::string::StringExtend;
use crate::new_less::token::lib::Token;
use crate::new_less::token::select::{TokenCombina, TokenSelect};

///
/// 选择器范式
///
#[derive(Debug, PartialEq, Clone)]
pub enum SelectParadigm {
  // 选择器
  SelectWrap(String),
  
  // 选择链接器
  CominaWrap(String),
  
  // 其他token
  OtherWrap(String),
  
  // * 通配符号
  NormalWrap(String),
}


#[derive(Debug, Clone)]
pub struct Selector {
  pub origin_txt: String,
  pub single_select_txt: Vec<String>,
}

impl Selector {
  ///
  /// 初始化方法
  ///
  pub fn new(txt: String) -> Result<Selector, String> {
    let mut obj = Selector {
      origin_txt: txt.trim().to_string(),
      single_select_txt: vec![],
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
  
  pub fn join(paradigm: Vec<SelectParadigm>) -> String {
    let mut base = "".to_string();
    for word_paradigm in paradigm {
      match word_paradigm {
        SelectParadigm::SelectWrap(cc) | SelectParadigm::CominaWrap(cc) | SelectParadigm::OtherWrap(cc) | SelectParadigm::NormalWrap(cc) => {
          base += &cc;
        }
      }
    }
    base
  }
  
  
  fn parse(&mut self) -> Result<(), String> {
    let charlist = self.origin_txt.tocharlist();
    let mut index = 0;
    let mut temp: String = "".to_string();
    let mut paradigm_vec: Vec<SelectParadigm> = vec![];
    let mut include_attr = false;
    
    // 循环解析
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
      
      // 跳过空格
      if Token::is_space_token(&char) && Token::is_space_token(&nextchar) {
        index += 1;
        continue;
      }
      // 有任务则继续填词
      if !Token::is_token(&char) {
        temp += &char.clone();
        if index + 1 != charlist.len() {
          index += 1;
          continue;
        }
      }
      
      // 报错信息
      let errormsg = |char: &str, index: &usize| -> Result<(), String> {
        Err(format!("select text {}, char {} is not allow,index is {}", self.origin_txt, char, index))
      };
      
      // 检查相邻 token
      let check_adjacent_token = |forbidword: Vec<&str>, char: &str, index: &usize| -> Result<(), String> {
        if Token::is_token(char) {
          // 验证 连接词 不能固定想连
          let res = forbidword.into_iter().find(|x| x == &char);
          match res {
            None => {}
            Some(_err_char) => {
              return errormsg(char, index);
            }
          }
        }
        Ok(())
      };
      
      
      if index == 0 {
        if Token::is_token(&char) {
          if charlist.len() == 1 && char != TokenSelect::WildCard.tostr_value() {
            return errormsg(&char, &index);
          }
          // 第一个词 是符号
          if TokenSelect::is(&char) {
            // 第一个词 是 选择符号
            match TokenSelect::try_from(char.clone().as_str()).unwrap() {
              TokenSelect::ClassToken | TokenSelect::IdToken => {
                temp += &char.clone();
                // 起始符 后续不能接 任意 词根符 类似 "#>" ".*"
                if Token::is_token(&nextchar) {
                  return errormsg(&nextchar, &(index + 1));
                }
              }
              TokenSelect::Colon => {
                temp += &char.clone();
                if nextchar != TokenSelect::Colon.tostr_value() && Token::is_token(&nextchar) {
                  return errormsg(&nextchar, &(index + 1));
                }
              }
              TokenSelect::AttrBegin => {
                include_attr = true;
                temp += &char.clone();
                // 起始符 后续不能接 任意 词根符 类似 "#>" ".*"
                if Token::is_token(&nextchar) {
                  return errormsg(&nextchar, &(index + 1));
                }
              }
              TokenSelect::AttrEnd => {
                return errormsg(&char, &index);
              }
              TokenSelect::WildCard => {
                paradigm_vec.push(SelectParadigm::NormalWrap("*".to_string()));
              }
            }
          } else if TokenCombina::is(&char) {
            // 第一个词 是 链接符号 不考虑空格
            match TokenCombina::try_from(char.clone().as_str()).unwrap() {
              TokenCombina::Comma => {
                return errormsg(&char, &index);
              }
              TokenCombina::ExtendChar => {
                paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::ExtendChar.tostr_value()));
                if !Token::is_space_token(&nextchar) {
                  paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value()));
                }
              }
              TokenCombina::ColumnChar => {
                index += 1;
                paradigm_vec.push(SelectParadigm::CominaWrap("||".to_string()));
              }
              TokenCombina::BrotherNextChar => {
                paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::BrotherNextChar.tostr_value()));
                // 补空格
                if !Token::is_space_token(&nextchar) {
                  paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value()));
                }
              }
              TokenCombina::BrotherMatchChar => {
                paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::BrotherMatchChar.tostr_value()));
                // 补空格
                if !Token::is_space_token(&nextchar) {
                  paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value()));
                }
              }
              _ => {}
            }
            match check_adjacent_token(vec!["\n", "\r", "]", ",", "~", "+", "|", "~", ">", "'", r#"""#], &nextchar, &(index + 1)) {
              Ok(_) => {}
              Err(msg) => {
                return Err(msg);
              }
            }
          } else {
            return errormsg(&char, &index);
          }
        } else {
          // 第一个词 非符号
          temp += &char.clone();
        }
      } else if index == charlist.len() - 1 {
        // 结尾处理
        if Token::is_token(&char) {} else {
          if !temp.is_empty() {
            paradigm_vec.push(SelectParadigm::SelectWrap(temp.clone()));
            temp = "".to_string();
          }
        }
        if !paradigm_vec.is_empty() {
          let single_select_txt = Self::join(paradigm_vec.clone());
          self.single_select_txt.push(single_select_txt);
          paradigm_vec = vec![];
        }
      } else {
        // 过程处理
        if Token::is_token(&char) {
          if !temp.is_empty() {
            paradigm_vec.push(SelectParadigm::SelectWrap(temp.clone()));
            temp = "".to_string();
          }
          if TokenSelect::is(&char) {
            // 词 是 选择符号
            match TokenSelect::try_from(char.clone().as_str()).unwrap() {
              TokenSelect::ClassToken | TokenSelect::IdToken => {
                temp += &char.clone();
                // 起始符 后续不能接 任意 词根符 类似 "#>" ".*"
                if Token::is_token(&nextchar) {
                  return errormsg(&nextchar, &(index + 1));
                }
              }
              TokenSelect::Colon => {
                temp += &char.clone();
                if nextchar != TokenSelect::Colon.tostr_value() && Token::is_token(&nextchar) {
                  return errormsg(&nextchar, &(index + 1));
                }
              }
              TokenSelect::AttrBegin => {
                if include_attr {
                  return errormsg(&char, &index);
                }
              }
              TokenSelect::AttrEnd => {
                if !include_attr {
                  return errormsg(&char, &index);
                }
              }
              TokenSelect::WildCard => {
                paradigm_vec.push(SelectParadigm::NormalWrap("*".to_string()));
              }
            }
          } else if TokenCombina::is(&char) {
            match TokenCombina::try_from(char.clone().as_str()).unwrap() {
              TokenCombina::Comma => {
                let single_select_txt = Self::join(paradigm_vec.clone());
                self.single_select_txt.push(single_select_txt);
                paradigm_vec = vec![];
              }
              TokenCombina::Space | TokenCombina::NewLineOs | TokenCombina::NewLineWindos => {
                if !Token::is_space_token(&prevchar) {
                  paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value()));
                }
                let space = SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value());
                if paradigm_vec.last().unwrap() != &space {
                  paradigm_vec.push(space);
                }
              }
              TokenCombina::ExtendChar => {
                if !Token::is_space_token(&nextchar) {
                  paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value()));
                }
                paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::ExtendChar.tostr_value()));
                if !Token::is_space_token(&nextchar) {
                  paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value()));
                }
              }
              TokenCombina::ColumnChar => {}
              TokenCombina::BrotherNextChar => {
                if !Token::is_space_token(&prevchar) {
                  paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value()));
                }
                paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::BrotherNextChar.tostr_value()));
                if !Token::is_space_token(&nextchar) {
                  paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value()));
                }
              }
              TokenCombina::BrotherMatchChar => {
                if !Token::is_space_token(&prevchar) {
                  paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value()));
                }
                paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::BrotherMatchChar.tostr_value()));
                if !Token::is_space_token(&nextchar) {
                  paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value()));
                }
              }
            }
            if Token::is_token(&nextchar) {
              // 验证 连接词 不能固定想连
              let error_token = vec!["\n", "\r", "]", ",", "~", "+", "|", "~", ":", ">", "'", r#"""#];
              let res = error_token.into_iter().find(|x| *x == &nextchar);
              match res {
                None => {}
                Some(_err_char) => {
                  return errormsg(&nextchar, &(index + 1));
                }
              }
            }
          } else {
            // 其他非关键词根 []
            
          }
        }
      }
      index += 1;
    }
    // println!("{:#?}", paradigm_vec);
    
    Ok(())
  }
}