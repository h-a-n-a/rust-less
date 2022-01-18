use crate::new_less::token::{Token, TokenSelect};
use strum::IntoEnumIterator;
use crate::extend::enum_extend::generic_iterator;

#[test]
fn test_token() {
  let res = TokenSelect::try_from(".").unwrap();
  for e in TokenSelect::iter() {
    println!("{:#?}", e.to_string());
  }
  
  generic_iterator::<TokenSelect, _, _>(|x| { x.to_string(); });
  assert_eq!(res, TokenSelect::ClassToken);
}