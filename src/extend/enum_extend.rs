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
}
