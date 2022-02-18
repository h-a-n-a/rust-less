pub struct Token(String);

///
/// 词根处理
///
impl Token {
  pub fn new(value: String) -> Self {
    Token(value)
  }

  pub fn get_token() -> Vec<String> {
    vec![
      ".", ",", "!", "?", "^", "#", "~", " ", "\n", "\r", "|", ":", "%", "$", "&", "[", "]", "@",
      "/", r"\", "+", ">", "<", "}", "{", "*", "-", "=", "`", "(", ")", ";", "'", r#"""#,
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect()
  }

  ///
  /// 是否是 词根
  ///
  pub fn is_token(char: &str) -> bool {
    Token::get_token().into_iter().any(|x| x == char)
  }

  ///
  /// 是否是空白字符串
  ///
  pub fn is_space_token(char: &str) -> bool {
    char == " " || char == "\n" || char == "\r"
  }
}
