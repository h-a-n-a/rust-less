use std::ops::Deref;
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::comment::skip_comment;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::{FileInfo, FileWeakRef};
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{NodeRef, StyleNode};
use crate::new_less::parse::RuleNode;

pub trait Rule {
  fn parse_rule(&mut self) -> Result<(), String>;
}

impl Rule for FileInfo {
  fn parse_rule(&mut self) -> Result<(), String> {
    let nodes = parse_rule(
      self.context.clone(),
      &self.origin_charlist,
      &self.locmap,
      self.self_weak.clone(),
    )?;
    nodes.iter().for_each(|node| {
      node.borrow().visit_mut_file(self);
    });
    self.block_node.append(
      &mut nodes
        .into_iter()
        .map(StyleNode::Rule)
        .collect::<Vec<StyleNode>>(),
    );
    Ok(())
  }
}

impl Rule for RuleNode {
  fn parse_rule(&mut self) -> Result<(), String> {
    let nodes = parse_rule(
      self.context.clone(),
      &self.origin_charlist,
      &self.locmap,
      self.file_info.clone(),
    )?;
    nodes.iter().for_each(|node| {
      node.borrow_mut().parent = self.weak_self.clone();
    });
    self.block_node.append(
      &mut nodes
        .into_iter()
        .map(StyleNode::Rule)
        .collect::<Vec<StyleNode>>(),
    );
    Ok(())
  }
}

fn parse_rule(
  context: ParseContext,
  origin_charlist: &Vec<char>,
  locmap: &Option<LocMap>,
  file_info: FileWeakRef,
) -> Result<Vec<NodeRef>, String> {
  let mut blocklist: Vec<NodeRef> = vec![];
  let mut templist: Vec<char> = vec![];
  let mut index = 0;

  // 块等级
  let mut braces_level = 0;
  // 结束标记 & 开始标记
  let endqueto = ';';
  let start_braces = '{';
  let end_braces = '}';

  let mut record_loc: Option<Loc> = None;
  let mut skipcall = skip_comment();
  let mut selector_txt: Vec<char> = vec![];

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
    if context.deref().borrow().option.sourcemap
      && char != ' '
      && char != '\r'
      && char != '\n'
      && record_loc.is_none()
    {
      record_loc = Some(locmap.as_ref().unwrap().get(&index).unwrap());
    }
    templist.push(char.clone());

    if char == start_braces {
      if braces_level == 0 {
        selector_txt = templist[0..templist.len() - 1].to_vec().trim();
        templist.clear();
      }
      braces_level += 1;
    }

    if char == endqueto && braces_level == 0 {
      templist.clear();
      record_loc = None;
    }

    if char == end_braces {
      braces_level -= 1;
      if braces_level == 0 {
        match RuleNode::new(
          templist[0..templist.len() - 1].to_vec().trim(),
          selector_txt.clone(),
          record_loc,
          file_info.clone(),
          context.clone(),
        ) {
          Ok(rule) => {
            blocklist.push(rule);
          }
          Err(msg) => {
            return Err(msg);
          }
        }
        selector_txt = vec![];
        templist.clear();
        record_loc = None;
      }
    }
    index += 1;
  }

  if braces_level != 0 {
    return Err("the content contains braces that are not closed!".to_string());
  }

  Ok(blocklist)
}
