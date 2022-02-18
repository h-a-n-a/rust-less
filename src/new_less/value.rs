use crate::extend::string::StringExtend;
use crate::new_less::scan::{traversal, ScanArg, ScanResult};
use derivative::Derivative;
use serde::Serialize;

#[derive(Derivative, Serialize, Clone)]
#[derivative(Debug)]
pub struct ValueNode {
  pub origin_txt: String,
  charlist: Vec<String>,
  pub word_ident_list: Vec<String>,
}

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

  LeftBrackets,

  RightBrackets,

  LeftParentheses,

  RightParentheses,
}

impl ValueNode {
  pub fn new(txt: String) -> Result<Self, String> {
    let mut obj = Self {
      origin_txt: txt,
      charlist: txt.tocharlist(),
      word_ident_list: vec![],
    };

    obj.parse()?;
    Ok(obj)
  }

  fn parse(&mut self) -> Result<(), String> {
    let charlist = &self.charlist.clone();
    if charlist.is_empty() {
      return Err("var declare text is empty".to_string());
    }
    let index = 1;
    let res = traversal(
      Some(*start),
      charlist,
      &mut (move |arg, charword| {
        let ScanArg {
          mut temp,
          index,
          mut hasend,
        } = arg;
        let (_, char, next) = charword;

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
