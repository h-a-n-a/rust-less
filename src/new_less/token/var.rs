use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;

#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenVarKeyAllow {
  #[strum(serialize = ":")]
  Colon,

  #[strum(serialize = "_")]
  Underline,

  #[strum(serialize = "-")]
  Dash,
}

#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenVarValue {
  #[strum(serialize = "+")]
  Add,

  #[strum(serialize = "-")]
  Sub,

  #[strum(serialize = "*")]
  Mult,

  #[strum(serialize = "/")]
  Divd,

  #[strum(serialize = "~")]
  Escape,

  #[strum(serialize = "(")]
  LeftBrackets,

  #[strum(serialize = ")")]
  RightBrackets,

  #[strum(serialize = r#"'"#)]
  Apos,

  #[strum(serialize = r#"""#)]
  Quote,

  #[strum(serialize = "#")]
  Anchor,
}

impl EnumExtend for TokenVarKeyAllow {}

impl StringInto for TokenVarKeyAllow {}

impl EnumExtend for TokenVarValue {}

impl StringInto for TokenVarValue {}
