use derivative::Derivative;
use serde::Serialize;

#[derive(Derivative, Serialize, Clone)]
#[derivative(Debug)]
pub struct ValueNode {
  pub origin_txt: String,
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
}

impl ValueNode {
  pub fn new(txt: String) -> Result<Self, String> {
    let mut obj = Self {
      origin_txt: txt,
      word_ident_list: vec![],
    };
    obj.parse()?;
    Ok(obj)
  }

  fn parse(&mut self) -> Result<(), String> {
    Ok(())
  }
}
