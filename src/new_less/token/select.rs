use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;

///
/// Select 合词字符串
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenSelect {
  #[strum(serialize = ".")]
  ClassToken,

  #[strum(serialize = "#")]
  IdToken,

  #[strum(serialize = "[")]
  AttrBegin,

  #[strum(serialize = "]")]
  AttrEnd,

  #[strum(serialize = "(")]
  LeftBrackets,

  #[strum(serialize = ")")]
  RightBrackets,

  #[strum(serialize = "*")]
  WildCard,

  #[strum(serialize = ":")]
  Colon,
}

///
/// Select 允许的连接符
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenCombina {
  #[strum(serialize = ",")]
  Comma,

  #[strum(serialize = " ")]
  Space,

  #[strum(serialize = "\n")]
  NewLineOs,

  #[strum(serialize = "\r")]
  NewLineWindos,

  #[strum(serialize = ">")]
  ExtendChar,

  #[strum(serialize = "|")]
  ColumnChar,

  #[strum(serialize = "+")]
  BrotherNextChar,

  #[strum(serialize = "~")]
  BrotherMatchChar,
}

///
/// Select 允许的 安全字符符
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenAllow {
  #[strum(serialize = r"\")]
  LeftSlant,

  #[strum(serialize = "_")]
  Underscore,

  #[strum(serialize = "-")]
  Dash,
}

///
/// & 表示当前选择器的父级
/// & -> $(&) -> 用于后期替换父元素
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenKeyWord {
  #[strum(serialize = "&")]
  PranedRefer,
}

impl EnumExtend for TokenSelect {}

impl EnumExtend for TokenCombina {}

impl EnumExtend for TokenAllow {}

impl EnumExtend for TokenKeyWord {}

impl StringInto for TokenSelect {}

impl StringInto for TokenCombina {}

impl StringInto for TokenAllow {}

impl StringInto for TokenKeyWord {}
