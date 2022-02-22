use std::fmt::{Debug, Formatter};
use crate::extend::string::StringExtend;
use crate::new_less::scan::{traversal, ScanArg, ScanResult};
use crate::new_less::token::lib::Token;
use serde::Serialize;
use crate::extend::enum_extend::EnumExtend;
use crate::new_less::ident::IdentType;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::token::value::TokenValueAllow;

#[derive(Serialize, Clone)]
pub struct ValueNode {
  // 原始字符
  pub origin_txt: String,

  // 字符 向量 只读
  charlist: Vec<String>,

  // 内部处理 地图
  #[serde(skip_serializing)]
  map: LocMap,

  // 单词 范式
  pub word_ident_list: Vec<IdentType>,
}

impl Debug for ValueNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ValueNode")
      .field("origin_txt", &self.origin_txt)
      .field("word_ident_list", &self.word_ident_list)
      .finish()
  }
}

impl ValueNode {
  pub fn new(txt: String, loc: Option<Loc>) -> Result<Self, String> {
    let map = if loc.is_none() {
      LocMap::new(txt.clone())
    } else {
      LocMap::merge(loc.as_ref().unwrap(), &txt).0
    };
    let mut obj = Self {
      origin_txt: txt.clone(),
      charlist: txt.tocharlist(),
      map,
      word_ident_list: vec![],
    };
    obj.parse()?;
    Ok(obj)
  }

  ///
  /// 报错信息
  ///
  pub fn error_msg(&self, index: &usize) -> String {
    let error_loc = self.map.get(index).unwrap();
    let char = self.charlist.get(*index).unwrap().to_string();
    format!(
      "text {}, char {} is not allow, line is {} col is {}",
      &self.origin_txt, char, error_loc.line, error_loc.col
    )
  }

  ///
  /// 产生代码
  ///
  pub fn code_gen(&self) {}

  ///
  /// 是否是数字
  ///
  pub fn is_number(char: &str) -> bool {
    char.parse::<i32>().is_ok()
  }

  ///
  /// 转化变量
  ///
  pub fn parse_value_var(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let res = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let ScanArg {
          mut temp,
          mut index,
          mut hasend,
        } = arg;
        let (_, char, nextchar) = charword;
        // 第一位必须是 @
        if temp.is_empty() && &char == "@" {
          temp += "@";
          Ok(ScanResult::Arg(ScanArg {
            index,
            temp,
            hasend,
          }))
        } else if temp.is_empty() {
          Err(self.error_msg(&index))
        } else {
          // 后续写词
          if Token::is_token(&char) {
            if &char == "-" {
              if Token::is_token(&nextchar) {
                hasend = true;
                index -= 1;
              } else if !nextchar.is_empty() {
                temp += &char;
              }
              // @- is error
              if temp.len() < 2 {
                return Err(self.error_msg(&index));
              }
            } else if &char == "+" || &char == "*" || &char == "/" || Token::is_space_token(&char) {
              // @+ @* is error
              if temp.len() < 2 {
                return Err(self.error_msg(&index));
              }
              hasend = true;
              index -= 1;
            } else if &char == r#"\"# {
              temp += &char;
            } else {
              return Err(self.error_msg(&index));
            }
          } else {
            temp += &char;
          }
          Ok(ScanResult::Arg(ScanArg {
            index,
            temp,
            hasend,
          }))
        }
      }),
    )?;
    Ok(res)
  }

  ///
  /// 转化 数值
  ///
  pub fn parse_value_number(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let res = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let ScanArg {
          temp,
          index,
          hasend,
        } = arg;
        let (_, char, _) = charword;
        let new_arg = ScanArg {
          index,
          temp,
          hasend,
        };
        Ok(ScanResult::Arg(new_arg))
      }),
    )?;
    Ok(res)
  }

  ///
  /// 判断 是否 是 操作符
  ///
  fn is_operator(char: &str) -> bool {
    vec!["+", "-", "*", "/"].contains(&char)
  }

  ///
  /// 转化
  ///
  fn parse(&mut self) -> Result<(), String> {
    let charlist = self.charlist.clone();
    if charlist.is_empty() {
      return Err("var declare text is empty".to_string());
    }
    let index: usize = 0;
    traversal(
      Some(index),
      &charlist,
      &mut (move |arg, charword| {
        let ScanArg {
          temp,
          mut index,
          hasend,
        } = arg;
        let (_, char, nextchar) = charword;

        // 处理空格
        if Token::is_space_token(&char) {
          match self.word_ident_list.last() {
            None => {}
            Some(val) => match val {
              IdentType::Space => {
                return Ok(ScanResult::Skip);
              }
              _ => {
                self.word_ident_list.push(IdentType::Space);
              }
            },
          }
        } else if &char == "@" {
          let (var, end) = self.parse_value_var(&index)?;
          self.word_ident_list.push(IdentType::Var(var));
          index = end;
        }
        // 处理prop
        else if &char == "$" {}
        // 处理 引用
        else if &char == "~" {}
        // 处理 keyword
        else if &char == "!" {}
        // 处理引号词
        else if &char == r#"""# {} else if &char == r#"'"# {}
        // 处理括号
        else if TokenValueAllow::is(&char) {
          if &char != r#"\"# {} else {
            return Ok(ScanResult::Skip);
          }
        }
        // 操作符
        else if Self::is_operator(&char) {
          if self.word_ident_list.is_empty() {
            return Ok(ScanResult::Skip);
          } else {
            let last_item = self.word_ident_list.last().unwrap();
            if last_item.is_number() || last_item.is_var() {
              self.word_ident_list.push(IdentType::Operator(char));
            } else if last_item.is_space() {
              if self.word_ident_list.len() > 1 {
                let before_last_item = self.word_ident_list.get(self.word_ident_list.len() - 2).unwrap();
                if before_last_item.is_var() || before_last_item.is_number() {
                  self.word_ident_list.push(IdentType::Operator(char));
                } else if before_last_item.is_operator() {
                  return Err(self.error_msg(&index));
                } else {
                  return Ok(ScanResult::Skip);
                }
              } else {
                return Ok(ScanResult::Skip);
              }
            } else {
              self.word_ident_list.push(IdentType::Space);
            }
          }
        }
        // 处理 数值
        else if Self::is_number(&char) {}
        // 处理单词
        else {}

        let new_arg = ScanArg {
          index,
          temp,
          hasend,
        };
        Ok(ScanResult::Arg(new_arg))
      }),
    )?;

    Ok(())
  }
}
