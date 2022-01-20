use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;
use crate::extend::string::StringExtend;
use crate::new_less::token::lib::Token;
use crate::new_less::token::select::{TokenCombina, TokenSelect};

///
/// 选择器范式
///
#[derive(Debug, PartialEq)]
pub enum SelectParadigm {
  // 选择器
  SelectWrap(String),

  // 选择链接器
  CominaWrap(String),

  // 其他token
  OtherWrap(String),

  // * 特殊符号
  NormalWrap(String),
}


#[derive(Debug, Clone)]
pub struct Selector {
  origin_txt: String,
  single_select_txt: Vec<String>,
}

impl Selector {
  ///
  /// 初始化方法
  ///
  pub fn new(txt: String) -> Result<Selector, String> {
    let obj = Selector {
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


  fn parse(&self) -> Result<(), String> {
    let charlist = self.origin_txt.tocharlist();
    let mut index = 0;
    let mut temp: String = "".to_string();
    let mut paradigm_vec: Vec<SelectParadigm> = vec![];
    let mut include_attr = false;

    /* 处理绝对不可能出现的字符 */
    let is_safe = |charlist: Vec<String>| -> &str{
      let never_exist_token = vec![";", "@", r#"\"#, r#"/"#];
      let mut safe = "";
      for char in charlist {
        let list_ref = &never_exist_token;
        match list_ref.into_iter().find(|x| { **x == char }) {
          None => {
            continue;
          }
          Some(match_char) => {
            safe = match_char.clone();
            break;
          }
        }
      }
      safe
    };

    let check_res = is_safe(charlist.clone());
    if !check_res.is_empty() {
      return Err(format!("select text {} is not allow is exist {}", self.origin_txt, check_res));
    }

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
      if index == 0 {
        if Token::is_token(&char) {
          // 第一个词 是符号
          if TokenSelect::is(&char) {
            // 第一个词 是 选择符号
            match TokenSelect::try_from(char.clone().as_str()).unwrap() {
              TokenSelect::ClassToken | TokenSelect::IdToken => {
                temp += &char.clone();
                // 起始符 后续不能接 任意 词根符 类似 "#>" ".*"
                if Token::is_token(&nextchar) {
                  return Err(format!("select text {},char {} is not allow,index is {}", self.origin_txt, &nextchar, index + 1));
                }
              }
              TokenSelect::AttrBegin => {
                include_attr = true;
                temp += &char.clone();
                // 起始符 后续不能接 任意 词根符 类似 "#>" ".*"
                if Token::is_token(&nextchar) {
                  return Err(format!("select text {},char {} is not allow,index is {}", self.origin_txt, &nextchar, index + 1));
                }
              }
              TokenSelect::AttrEnd => {
                return Err(format!("select text {},char {} is not allow,index is {}", self.origin_txt, "]", index));
              }
              TokenSelect::WildCard => {
                paradigm_vec.push(SelectParadigm::NormalWrap("*".to_string()));
              }
            }
          } else if TokenCombina::is(&char) {
            // 第一个词 是 链接符号 不考虑空格
            match TokenCombina::try_from(char.clone().as_str()).unwrap() {
              TokenCombina::Comma => {
                return Err(format!("select text {},char {} is not allow,index is {}", self.origin_txt, &char, index));
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
                if !Token::is_space_token(&nextchar) {
                  paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value()));
                }
              }
              TokenCombina::BrotherMatchChar => {
                paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::BrotherMatchChar.tostr_value()));
                if !Token::is_space_token(&nextchar) {
                  paradigm_vec.push(SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value()));
                }
              }
              _ => {}
            }
            if Token::is_token(&nextchar) {
              // 验证 连接词 不能固定想连
              let error_token = vec!["\n", "\r", "]", ",", "~", "+", "|", "~", ":", ">", "'", r#"""#];
              let res = error_token.into_iter().find(|x| *x == &nextchar);
              match res {
                None => {}
                Some(_err_char) => {
                  return Err(format!("select text {},char {} is not allow,index is {}", self.origin_txt, &nextchar, index + 1));
                }
              }
            }
          } else {
            return Err(format!("select text {},char {} is not allow,index is {}", self.origin_txt, &char, index));
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
                  return Err(format!("select text {},char {} is not allow,index is {}", self.origin_txt, &nextchar, index + 1));
                }
              }
              TokenSelect::AttrBegin => {
                if include_attr {
                  return Err(format!("select text {},char {} is not allow,index is {}", self.origin_txt, &char, index));
                }
              }
              TokenSelect::AttrEnd => {
                if !include_attr {
                  return Err(format!("select text {},char {} is not allow,index is {}", self.origin_txt, &char, index));
                }
              }
              TokenSelect::WildCard => {
                paradigm_vec.push(SelectParadigm::NormalWrap("*".to_string()));
              }
            }
          } else if TokenCombina::is(&char) {
            match TokenCombina::try_from(char.clone().as_str()).unwrap() {
              TokenCombina::Comma => {}
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
                if nextchar != TokenCombina::ColumnChar.tostr_value() {
                  return Err(format!("select text {},char {} is not allow,index is {}", self.origin_txt, &nextchar, index + 1));
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
                  return Err(format!("select text {},char {} is not allow,index is {}", self.origin_txt, &nextchar, index + 1));
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
    println!("{:#?}", paradigm_vec);

    Ok(())
  }
}