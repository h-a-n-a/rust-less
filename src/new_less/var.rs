use crate::extend::vec_str::VecStrExtend;
use crate::new_less::block::OriginBlock;
use crate::new_less::comment::skip_comment;
use crate::new_less::fileinfo::FileInfo;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::option::ParseOption;

pub trait Var {
  fn parse_var(&self) -> Result<Vec<OriginBlock>, String>;
  fn parse_import(&self) -> Result<Vec<OriginBlock>, String>;
}

impl Var for FileInfo {
  fn parse_var(&self) -> Result<Vec<OriginBlock>, String> {
    parse_var(&self.option, &self.origin_charlist, &self.locmap, true)
  }
  
  fn parse_import(&self) -> Result<Vec<OriginBlock>, String> {
    parse_var(&self.option, &self.origin_charlist, &self.locmap, false)
  }
}

impl Var for OriginBlock {
  fn parse_var(&self) -> Result<Vec<OriginBlock>, String> {
    parse_var(&self.option, &self.origin_charlist, &self.locmap, false)
  }
  
  fn parse_import(&self) -> Result<Vec<OriginBlock>, String> {
    Ok(vec![])
  }
}

///
/// 转化当前层变量
///
fn parse_var(options: &ParseOption, origin_charlist: &Vec<String>, locmap: &Option<LocMap>, istop: bool) -> Result<Vec<OriginBlock>, String> {
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
  let mut skipcall = skip_comment();
  
  while index < origin_charlist.len() {
    let char = origin_charlist.get(index).unwrap().clone();
    let word = origin_charlist.try_getword(index, 2).unwrap();
    
    let prev_index = index;
    let skip_res = skipcall(word, char.clone(), &mut index);
    if skip_res || prev_index != index {
      record_loc = None;
      index += 1;
      continue;
    }
    
    // 记录第一个非空字符 起始位置
    if options.sourcemap && char != " " && char != "\r" && char != "\n" && record_loc.is_none() {
      record_loc = Some(locmap.as_ref().unwrap().get(index).unwrap());
    }
    templist.push(char.clone());
    if char == endqueto && braces_level == 0 {
      if templist.join("").trim().to_string() == "" {
        return if options.sourcemap {
          Err(format!("multiple semicolons appear in the current scope,loc at {:#?}", locmap.as_ref().unwrap().get(index).unwrap()))
        } else {
          Err(format!("multiple semicolons appear in the current scope,word order is {}", index, ))
        }
      }
      templist.clear();
      record_loc = None;
    }
    
    // ignore 忽略 大括号区域
    if char == start_braces {
      braces_level += 1;
    }
    if char == end_braces {
      braces_level -= 1;
      if braces_level == 0 {
        templist.clear();
        record_loc = None;
      }
    }
    
    // 最后检查 分号闭合情况
    if index == origin_charlist.len() - 1 {
      let checkstr = templist.join("").trim().to_string();
      if !checkstr.is_empty() {
        return Err(format!("the word is not with endqueto -> {}", checkstr));
      }
    }
    
    index += 1;
  }
  
  if braces_level != 0 {
    return Err("the content contains braces that are not closed!".to_string());
  }
  
  Ok(blocklist)
}

///
/// 转化当前层 引用变量
/// 只有基础层 会调用
///
fn parse_import(options: &ParseOption, origin_charlist: &Vec<String>, locmap: &Option<LocMap>) -> Result<Vec<OriginBlock>, String> {
  let mut blocklist: Vec<OriginBlock> = vec![];
  Ok(blocklist)
}