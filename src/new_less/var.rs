use crate::extend::vec_str::VecStrExtend;
use crate::new_less::comment::skip_comment;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::{FileInfo, FileRef, FileWeakRef};
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{NodeWeakRef, StyleNode, VarRuleNode};
use crate::new_less::parse::RuleNode;

pub trait Var {
  fn parse_var(&mut self) -> Result<(), String>;
}

impl Var for FileInfo {
  fn parse_var(&mut self) -> Result<(), String> {
    let mut importfiles: Vec<FileRef> = vec![];
    let nodes = parse_var(
      self.context.clone(),
      &self.origin_charlist,
      &self.locmap,
      None,
      self.self_weak.clone(),
      &mut importfiles,
    )?;
    self.block_node.append(
      &mut nodes
        .into_iter()
        .map(StyleNode::Var)
        .collect::<Vec<StyleNode>>(),
    );
    self.import_files = importfiles;
    Ok(())
  }
}

impl Var for RuleNode {
  fn parse_var(&mut self) -> Result<(), String> {
    let mut importfiles: Vec<FileRef> = vec![];
    let nodes = parse_var(
      self.context.clone(),
      &self.origin_charlist,
      &self.locmap,
      self.weak_self.clone(),
      self.file_info.clone(),
      &mut importfiles,
    )?;
    self.block_node.append(
      &mut nodes
        .into_iter()
        .map(StyleNode::Var)
        .collect::<Vec<StyleNode>>(),
    );
    Ok(())
  }
}

///
/// 转化当前层变量
///
fn parse_var(
  context: ParseContext,
  origin_charlist: &Vec<char>,
  locmap: &Option<LocMap>,
  parent: NodeWeakRef,
  fileinfo: FileWeakRef,
  importfiles: &mut Vec<FileRef>,
) -> Result<Vec<VarRuleNode>, String> {
  let mut blocklist: Vec<VarRuleNode> = vec![];
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
    if context.borrow().option.sourcemap
      && char != ' '
      && char != '\r'
      && char != '\n'
      && record_loc.is_none()
    {
      record_loc = Some(locmap.as_ref().unwrap().get(&index).unwrap());
    }

    templist.push(char.clone());
    if char == endqueto && braces_level == 0 {
      let style_var = match VarRuleNode::new(
        templist.poly().trim().to_string(),
        record_loc,
        parent.clone(),
        fileinfo.clone(),
        context.clone(),
        importfiles,
      ) {
        Ok(obj) => obj,
        Err(msg) => {
          return Err(msg);
        }
      };
      blocklist.push(style_var);
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
      let checkstr = templist.poly().trim().to_string();
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
