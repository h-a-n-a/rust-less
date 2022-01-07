use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Loc {
  pub line: usize,
  pub col: usize,
  pub char: String,
  pub index: usize,
}


pub struct LocMap {
  data: HashMap<usize, Loc>,
}