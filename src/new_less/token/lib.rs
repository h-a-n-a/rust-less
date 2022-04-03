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
    if char.is_none() {
      false
    } else {
      Token::get_token().into_iter().any(|x| x == *char.unwrap())
    }
  }

  ///
  /// 是否是空白字符串
  ///
  pub fn is_space_token(char: Option<&char>) -> bool {
    if char.is_none() {
      false
    } else {
      *char.unwrap() == ' ' || *char.unwrap() == '\n' || *char.unwrap() == '\r'
    }
  }
}
