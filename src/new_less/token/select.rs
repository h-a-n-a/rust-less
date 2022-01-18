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
  
  #[strum(serialize = "*")]
  WildCard,
}


///
/// Select 允许的连接符
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenComina {
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
  
  #[strum(serialize = "||")]
  ColumnChar,
  
  #[strum(serialize = "+")]
  BrotherNextChar,
  
  #[strum(serialize = "~")]
  BrotherMatchChar,
}

impl EnumExtend for TokenSelect {}

impl EnumExtend for TokenComina {}

impl StringInto for TokenSelect {}

impl StringInto for TokenComina {}
