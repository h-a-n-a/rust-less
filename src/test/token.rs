use crate::extend::enum_extend::{EnumExtend};
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::token::lib::Token;
use crate::new_less::token::select::{SelectTokenParse, TokenSelect};

#[test]
fn test_enum_to_vec() {
  let list = TokenSelect::enum_vec();
  assert_eq!(list.poly(), ".#[]*");
}

#[test]
fn test_token_select_forbidden() {
  let list = Token::token_selector_forbidden();
  println!("{:#?}", list);
  println!(".....")
}