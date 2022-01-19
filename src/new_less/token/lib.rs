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