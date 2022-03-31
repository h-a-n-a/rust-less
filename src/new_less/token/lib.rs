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
      '/', '+', '>', '<', '}', '{', '*', '-', '=', '`', '(', ')', ';', '\'', '"','\\',
    ]
  }

  ///
  /// 是否是 词根
  ///
  pub fn is_token(char: &char) -> bool {
    Token::get_token().into_iter().any(|x| x == char)
  }

  ///
  /// 是否是空白字符串
  ///
  pub fn is_space_token(char: &char) -> bool {
    char == ' ' || char == '\n' || char == '\r'
  }
}
