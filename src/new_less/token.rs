use strum::{EnumIter, IntoEnumIterator};

pub struct Token(String);

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


///
/// 词根处理
///
impl Token {
  pub fn new(char: String) -> Result<Token, String> {
    Ok(Token(char))
  }
  
  pub fn get_token() -> Vec<String> {
    vec![
      ".",
      ",",
      "#",
      "~",
      " ",
      "\n",
      "\r",
      "|",
      ":",
      "[",
      "]",
      "@",
      "/",
      r"\",
      "+",
      ">",
      "*",
      "-",
      "_",
      "(",
      ")",
      ";",
      "'",
      r#"""#,
    ].into_iter()
      .map(|x| x.to_string()).collect()
  }
  
  pub fn token_selector_forbidden() {
    // let tokenlist = Token::get_token();
    // TokenSelect::
  }
  
  ///
  /// 是否是 词根
  ///
  pub fn is_token(char: &str) -> bool {
    match Token::get_token().into_iter().find(|x| { x == char }) {
      None => { false }
      Some(_) => { true }
    }
  }
  
  ///
  /// 是否是空白字符串
  ///
  pub fn is_space_token(char: &str) -> bool {
    if char == " " || char == "\n" || char == "\r" {
      true
    } else {
      false
    }
  }
}