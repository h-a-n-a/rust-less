pub mod select;

pub struct Token(String);


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
  
  pub fn token_selector_forbidden() -> Vec<String> {
    let tokenlist = Token::get_token();
    let mut list_select = TokenSelect::enum_vec();
    let mut list_combina = TokenComina::enum_vec();
    let mut list = vec![];
    list.append(&mut list_select);
    list.append(&mut list_combina);
    let mut other = vec![];
    tokenlist.into_iter().for_each(|token| {
      match list.clone().into_iter().find(|x| *x == token) {
        None => {
          other.push(token);
        }
        Some(_) => {}
      }
    });
    other
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