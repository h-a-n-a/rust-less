use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::String;
use crate::extend::string::StringExtend;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loc {
  pub line: usize,
  pub col: usize,
  pub char: String,
  pub index: usize,
}

#[derive(Debug, Clone)]
pub struct LocMap {
  data: HashMap<usize, Loc>,
}

impl LocMap {
  ///
  /// 初始化对象
  /// 根据传入的 字符串 txt 构造索引 行|列
  ///
  pub fn new(txt: String) -> LocMap {
    let chars = txt.tocharlist();
    let map = HashMap::new();
    let mut line = 1;
    let mut col = 1;
    let mut obj = LocMap {
      data: map
    };
    for (index, cc) in chars.iter().enumerate() {
      let loc: Loc;
      if *cc != '\r'.to_string() && *cc != '\n'.to_string() {
        loc = Loc {
          col,
          line,
          char: cc.clone(),
          index,
        };
        col += 1;
      } else {
        loc = Loc {
          col,
          line,
          char: cc.clone(),
          index,
        };
        col = 1;
        line += 1;
      }
      obj.data.insert(index, loc);
    }
    obj
  }

  pub fn get(&self, index: usize) -> Option<Loc> {
    match self.data.get(&index) {
      None => { None }
      Some(val) => { Some(val.clone()) }
    }
  }

  pub fn getloc(&self, line: usize, col: usize) -> Option<Loc> {
    let mut loc: Option<Loc> = None;
    for (_index, (_, map)) in self.data.iter().enumerate() {
      if map.line == line && map.col == col {
        loc = Some(map.clone());
        break;
      }
    }
    loc
  }
}