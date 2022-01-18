use crate::new_less::token::{Token, TokenSelect};

#[test]
fn test_token() {
  let res = TokenSelect::try_from(".").unwrap();
  assert_eq!(res, TokenSelect::ClassToken);
}