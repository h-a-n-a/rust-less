///
/// Select 合词字符串
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenImport {
  #[strum(serialize = r#"'"#)]
  Apost,

  #[strum(serialize = r#"""#)]
  Quote,
}
