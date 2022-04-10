use std::slice::Iter;
use crate::new_less::token::lib::TokenInterface;

#[derive(Debug)]
pub enum TokenSelectChar {
  ClassToken,
  IdToken,
  AttrBegin,
  AttrEnd,
  LeftBrackets,
  RightBrackets,
  WildCard,
  Colon,
}

impl TokenInterface for TokenSelectChar {
  fn to_str(&self) -> char {
    match self {
      TokenSelectChar::ClassToken => '.',
      TokenSelectChar::IdToken => '#',
      TokenSelectChar::AttrBegin => '[',
      TokenSelectChar::AttrEnd => ']',
      TokenSelectChar::LeftBrackets => '(',
      TokenSelectChar::RightBrackets => ')',
      TokenSelectChar::WildCard => '*',
      TokenSelectChar::Colon => ':'
    }
  }

  fn iterator() -> Iter<'static, TokenSelectChar> {
    static TOKENS: [TokenSelectChar; 8] = [
      TokenSelectChar::ClassToken,
      TokenSelectChar::IdToken,
      TokenSelectChar::AttrBegin,
      TokenSelectChar::AttrEnd,
      TokenSelectChar::LeftBrackets,
      TokenSelectChar::RightBrackets,
      TokenSelectChar::WildCard,
      TokenSelectChar::Colon
    ];
    TOKENS.iter()
  }

  fn is(cc: &char) -> bool {
    for token in Self::iterator() {
      if *cc == token.to_str() {
        return true;
      }
    }
    false
  }
}

///
/// Select 允许的连接符
///
#[derive(Debug)]
pub enum TokenCombinaChar {
  Comma,
  Space,
  NewLineOs,
  NewLineWindos,
  ExtendChar,
  ColumnChar,
  BrotherNextChar,
  BrotherMatchChar,
}


impl TokenInterface for TokenCombinaChar {
  fn to_str(&self) -> char {
    match self {
      TokenCombinaChar::Comma => ',',
      TokenCombinaChar::Space => ' ',
      TokenCombinaChar::NewLineOs => '\n',
      TokenCombinaChar::NewLineWindos => '\r',
      TokenCombinaChar::ExtendChar => '>',
      TokenCombinaChar::ColumnChar => '|',
      TokenCombinaChar::BrotherNextChar => '|',
      TokenCombinaChar::BrotherMatchChar => '~'
    }
  }

  fn iterator() -> Iter<'static, TokenCombinaChar> {
    static TOKENS: [TokenCombinaChar; 8] = [
      TokenCombinaChar::Comma,
      TokenCombinaChar::Space,
      TokenCombinaChar::NewLineOs,
      TokenCombinaChar::NewLineWindos,
      TokenCombinaChar::ExtendChar,
      TokenCombinaChar::ColumnChar,
      TokenCombinaChar::BrotherNextChar,
      TokenCombinaChar::BrotherMatchChar
    ];
    TOKENS.iter()
  }

  fn is(cc: &char) -> bool {
    for token in Self::iterator() {
      if *cc == token.to_str() {
        return true;
      }
    }
    false
  }
}

#[derive(Debug)]
pub enum TokenAllowChar {
  LeftSlant,
  Underscore,
  Dash,
}


impl TokenInterface for TokenAllowChar {
  fn to_str(&self) -> char {
    match self {
      TokenAllowChar::LeftSlant => '\\',
      TokenAllowChar::Underscore => '_',
      TokenAllowChar::Dash => '-',
    }
  }

  fn iterator() -> Iter<'static, TokenAllowChar> {
    static TOKENS: [TokenAllowChar; 3] = [
      TokenAllowChar::LeftSlant,
      TokenAllowChar::Underscore,
      TokenAllowChar::Dash
    ];
    TOKENS.iter()
  }

  fn is(cc: &char) -> bool {
    for token in Self::iterator() {
      if *cc == token.to_str() {
        return true;
      }
    }
    false
  }
}


#[derive(Debug)]
pub enum TokenKeyWordChar {
  PranedRefer,
  VarRefer,
}

impl TokenInterface for TokenKeyWordChar {
  fn to_str(&self) -> char {
    match self {
      TokenKeyWordChar::PranedRefer => '&',
      TokenKeyWordChar::VarRefer => '@',
    }
  }

  fn iterator() -> Iter<'static, TokenKeyWordChar> {
    static TOKENS: [TokenKeyWordChar; 2] = [
      TokenKeyWordChar::PranedRefer,
      TokenKeyWordChar::VarRefer
    ];
    TOKENS.iter()
  }

  fn is(cc: &char) -> bool {
    for token in Self::iterator() {
      if *cc == token.to_str() {
        return true;
      }
    }
    false
  }
}


