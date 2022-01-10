use crate::extend::string::StringExtend;
use crate::new_less::block::{OriginBlock, OriginBlockType};
use crate::new_less::loc::{Loc, LocMap};

pub fn parse_origin_block(content: String) -> Result<Vec<OriginBlock>, String> {
  let charlist = content.tocharlist();
  let mut blocklist: Vec<OriginBlock> = vec![];
  let mut templist: Vec<String> = vec![];
  let mut index = 0;

  // 块等级
  let mut braces_level = 0;

  // 结束标记 & 开始标记
  let endqueto = ";".to_string();
  let start_braces = "{".to_string();
  let end_braces = "}".to_string();
  //
  let locmap = LocMap::new(content);
  let mut record_loc: Option<Loc> = None;

  while index < charlist.len() {
    // 处理字符
    let char = charlist.get(index).unwrap().clone();
    if char != "\r" && char != "\n" && record_loc.is_none() {
      record_loc = Some(locmap.get(index).unwrap());
    }
    templist.push(char.clone());
    // 进入 style_list 中 块级内容 延迟后续进行 -> 递归计算
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
    // style_list 外部的内容 进行 变量 | 引用 | 注释 的标准计算
    if char == endqueto && braces_level == 0 {
      blocklist.push(OriginBlock {
        block_type: OriginBlockType::Var,
        content: templist.join(""),
        origin_charlist: vec![],
        loc: record_loc.unwrap(),
        level: 0,
        locmap: None,
        option: Default::default(),
        parent: None,
        block_node: None
      });
      templist.clear();
      record_loc = None;
    }
    index += 1;
  }

  if braces_level != 0 {
    return Err("the content contains braces that are not closed!".to_string());
  }

  Ok(blocklist)
}