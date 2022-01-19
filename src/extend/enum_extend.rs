use strum::IntoEnumIterator;

pub trait EnumExtend {
  fn enum_vec() -> Vec<String>
    where
      Self: IntoEnumIterator,
      Self: std::fmt::Display
  {
    let mut list = vec![];
    for e in Self::iter() {
      list.push(e.to_string());
    }
    list
  }
  
  fn is(char: &str) -> bool
    where
      Self: IntoEnumIterator,
      Self: std::fmt::Display
  {
    let list = Self::enum_vec();
    match list.into_iter().find(|x| { *x == char }) {
      None => {
        false
      }
      Some(_) => {
        true
      }
    }
  }
}
