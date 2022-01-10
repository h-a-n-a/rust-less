use crate::new_less::block::OriginBlock;
use crate::new_less::fileinfo::FileInfo;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::option::{OptionExtend, ParseOption};

pub trait Rule {
  fn parse_comment(&self) -> Result<Vec<OriginBlock>, String>;
}

impl Rule for FileInfo {
  fn parse_comment(&self) -> Result<Vec<OriginBlock>, String> {
    parse_comment(self.get_options(), &self.origin_charlist, &self.locmap)
  }
}


fn parse_comment(options: &ParseOption, origin_charlist: &Vec<String>, locmap: &Option<LocMap>) -> Result<Vec<OriginBlock>, String> {
  let mut blocklist: Vec<OriginBlock> = vec![];
  let mut templist: Vec<String> = vec![];
  let mut index = 0;
  
  // 块等级
  let mut braces_level = 0;
  // 结束标记 & 开始标记
  let endqueto = ";".to_string();
  let start_braces = "{".to_string();
  let end_braces = "}".to_string();
  
  let mut record_loc: Option<Loc> = None;
  
  while index < origin_charlist.len() {
    let char = origin_charlist.get(index).unwrap().clone();
    
    if char != "\r" && char != "\n" && record_loc.is_none() {
      record_loc = Some(locmap.get(index).unwrap());
    }
    templist.push(char.clone());
    
    if char == start_braces {
      braces_level += 1;
    }
    
    if char == endqueto && braces_level == 0 {
      templist.clear();
    }
    
    if char == end_braces {
      braces_level -= 1;
      if braces_level == 0 {
        let rule = OriginBlock::create_rule(
          templist.join(""),
          record_loc.unwrap(),
          None,
          options.clone(),
          None,
        );
        blocklist.push(rule);
        templist.clear();
        record_loc = None;
      }
    }
    
    index += 1;
  }
  
  
  Ok(blocklist);
}