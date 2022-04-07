pub struct Token(String);

///
/// 词根处理
///
impl Token {
  pub fn new(value: String) -> Self {
    Token(value)
  }

  pub fn get_token() -> Vec<char> {
    vec![
      '.', ',', '!', '?', '^', '#', '~', ' ', '\n', '\r', '|', ':', '%', '$', '&', '[', ']', '@',
      '/', '+', '>', '<', '}', '{', '*', '-', '=', '`', '(', ')', ';', '\'', '"', '\\',
    ]
  }

  ///
  /// 是否是 词根
  ///
  pub fn is_token(char: Option<&char>) -> bool {
    if let Some(cc) = char {
      Token::get_token().into_iter().any(|x| x == *cc)
    } else {
      false
    }
  }

  ///
  /// 是否是空白字符串
  ///
  pub fn is_space_token(char: Option<&char>) -> bool {
    if let Some(cc) = char {
      *cc == ' ' || *cc == '\n' || *cc == '\r'
    } else {
      false
    }
  }
}
