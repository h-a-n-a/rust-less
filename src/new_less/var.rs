use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::comment::skip_comment;
use crate::new_less::fileinfo::FileInfo;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::option::ParseOption;
use crate::new_less::parse::RuleNode;
use serde::Serialize;

pub trait Var {
  fn parse_var(&self) -> Result<Vec<VarNode>, String>;
}

#[derive(Debug, Clone, Serialize)]
pub struct VarNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Loc,
}

impl Var for FileInfo {
  fn parse_var(&self) -> Result<Vec<VarNode>, String> {
    parse_var(&self.option, &self.origin_charlist, &self.locmap, true)
  }
}

impl Var for RuleNode {
  fn parse_var(&self) -> Result<Vec<VarNode>, String> {
    parse_var(&self.option, &self.origin_charlist, &self.locmap, false)
  }
}

///
/// 检查是否 合规
/// 检查是否 变量
///
fn is_var(charlist: &[String], istop: bool, locationmsg: String) -> Result<bool, String> {
  // 变量片段中 含有换行
  if charlist.is_empty() {
    return Err(format!("var token word is empty,{}", locationmsg));
  }
  if charlist.iter().filter(|&x| x.as_str() == "\n" || x.as_str() == "\r").count() > 0 {
    return Err(format!(r#"token word has contains "\n","\r",{} "#, locationmsg));
  }
  // 变量类似 ;; || @a:10px;;
  if charlist[0].as_str() == ";" {
    return Err(format!(r#"token word is only semicolon,{} "#, locationmsg));
  }
  if istop {
    // 变量片段中首位必须是 @
    if charlist[0].as_str() != "@" {
      return Err(format!(r#"token word is not with @ begin,{} "#, locationmsg));
    }
    // 先判断 是否含有 @import
    if charlist.join("").indexOf("@import", None) > -1 {
      return Ok(false);
    }
    // 判断是否复合基本格式
    if charlist.join("").split(':').count() != 2 {
      return Err(format!(r#"var token is not liek '@var: 10px',{} ,{}"#, charlist.join(""), locationmsg));
    }
  }
  Ok(true)
}

///
/// 转化当前层变量
///
fn parse_var(options: &ParseOption, origin_charlist: &[String], locmap: &Option<LocMap>, istop: bool) -> Result<Vec<VarNode>, String> {
  let mut blocklist: Vec<VarNode> = vec![];
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
  
  let getmsg = |index: usize| -> String {
    let location_msg = if options.sourcemap {
      format!("loc at {:#?}", locmap.as_ref().unwrap().get(index).unwrap())
    } else {
      format!("word order is {}", index)
    };
    location_msg
  };
  
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
      let pure_text = templist.join("").trim().to_string().tocharlist();
      match is_var(&pure_text, istop, getmsg(index)) {
        Ok(val) => {
          if val {
            let style_var = VarNode {
              content: pure_text.join(""),
              loc: record_loc.unwrap(),
            };
            blocklist.push(style_var);
          }
        }
        Err(msg) => {
          return Err(msg);
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
