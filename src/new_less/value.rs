use crate::extend::string::StringExtend;
use crate::new_less::scan::{traversal, ScanArg, ScanResult};
use crate::new_less::token::lib::Token;
use derivative::Derivative;
use serde::Serialize;

#[derive(Derivative, Serialize, Clone)]
#[derivative(Debug)]
pub struct ValueNode {
  pub origin_txt: String,
  charlist: Vec<String>,
  pub word_ident_list: Vec<IdentType>,
}

#[derive(Derivative, Clone, Serialize)]
#[derivative(Debug)]
pub enum IdentType {
  // 10px 100% 100vh
  Number(String, Option<String>),
  // + - * /
  Operator(String),
  // @abc
  Var(String),
  // $abc
  Prop(String),
  // @{abc}
  InsertVar(String),
  // "abc"
  StringConst(String),
  // solid
  Word(String),
  // #abc17fc
  Color(String),
  // !important
  KeyWord(String),
  // " " ,"\n"
  Space,
  //  ~"(min-width: 768px)" (min-width: 768px) -> Only for MediaRule
  Escaping(String),
}

impl ValueNode {
  pub fn new(txt: String) -> Result<Self, String> {
    let mut obj = Self {
      origin_txt: txt.clone(),
      charlist: txt.tocharlist(),
      word_ident_list: vec![],
    };
    obj.parse()?;
    Ok(obj)
  }

  ///
  /// 是否是数字
  ///
  pub fn is_number(char: &str) -> bool {
    char.parse::<i32>().is_ok()
  }

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
          index,
          hasend,
        } = arg;
        let (_, char, _) = charword;
        if Token::is_token(&char) {
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
          }
        } else if &char == "@" {
        } else if &char == "$" {
        } else if &char == "~" {
        } else if Self::is_number(&char) {
        } else {
        }

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
