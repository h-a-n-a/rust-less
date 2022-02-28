use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
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
  // ( ) [ ] 计算运算可能性
  Brackets(String),
}

impl IdentType {
  pub fn is_number(&self) -> bool {
    matches!(self, IdentType::Number(_, _))
  }

  pub fn is_space(&self) -> bool {
    matches!(self, IdentType::Space)
  }

  pub fn is_operator(&self) -> bool {
    matches!(self, IdentType::Operator(..))
  }

  pub fn is_var(&self) -> bool {
    matches!(self, IdentType::Var(..))
  }
}

///
/// 标识符 词性
///
#[derive(Clone, Serialize, Debug)]
pub enum IdentNature {
  Space(IdentType),
  Calc(Vec<IdentType>),
  Word(IdentType),
}
