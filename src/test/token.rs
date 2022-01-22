use crate::extend::enum_extend::{EnumExtend};
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::token::select::{TokenSelect};

#[test]
fn test_enum_to_vec() {
  let list = TokenSelect::enum_vec();
  assert_eq!(list.poly(), ".#[]()*:");
}
