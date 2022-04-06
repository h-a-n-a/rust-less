use crate::extend::vec_str::VecStrExtend;
use crate::new_less::fileinfo::FileInfo;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::StyleNode;
use crate::new_less::option::{OptionExtend, ParseOption};
use crate::new_less::parse::RuleNode;
use serde::Serialize;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

pub trait Comment {
  fn parse_comment(&mut self) -> Result<(), String>;
  fn get_comment_blocknode(&self) -> Vec<CommentNode>;
  fn rm_comment(&self) -> String;
  fn skip_comment() -> Box<dyn FnMut(String, char, &mut usize) -> bool>;
}

#[derive(Clone, Serialize)]
pub struct CommentNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Option<Loc>,
  // 注释开始索引
  #[serde(skip_serializing)]
  startindex: usize,
}

impl Debug for CommentNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("CommentNode")
      .field("content", &self.content)
      .field("loc", &self.loc)
      .finish()
  }
}

impl Comment for FileInfo {
  fn parse_comment(&mut self) -> Result<(), String> {
    let nodes = parse_comment(&self.get_options(), &self.origin_charlist, &self.locmap)?;
    self.block_node.append(
      &mut nodes
        .into_iter()
        .map(StyleNode::Comment)
        .collect::<Vec<StyleNode>>(),
    );
    Ok(())
  }

  fn get_comment_blocknode(&self) -> Vec<CommentNode> {
    get_comment_blocknode(&self.block_node)
  }

  fn rm_comment(&self) -> String {
    let list = &self.get_comment_blocknode();
    if !list.is_empty() {
      rm_comment(list, &self.origin_charlist)
    } else {
      self.origin_txt_content.clone()
    }
  }

  fn skip_comment() -> Box<dyn FnMut(String, char, &mut usize) -> bool> {
    skip_comment()
  }
}

impl Comment for RuleNode {
  fn parse_comment(&mut self) -> Result<(), String> {
    let nodes = parse_comment(&self.get_options(), &self.origin_charlist, &self.locmap)?;
    self.block_node.append(
      &mut nodes
        .into_iter()
        .map(StyleNode::Comment)
        .collect::<Vec<StyleNode>>(),
    );
    Ok(())
  }

  fn get_comment_blocknode(&self) -> Vec<CommentNode> {
    get_comment_blocknode(&self.block_node)
  }

  fn rm_comment(&self) -> String {
    let node_list = &self.get_comment_blocknode();
    if !node_list.is_empty() {
      rm_comment(node_list, &self.origin_charlist)
    } else {
      self.origin_charlist.poly()
    }
  }

  fn skip_comment() -> Box<dyn FnMut(String, char, &mut usize) -> bool> {
    skip_comment()
  }
}

///
/// 获取一段 文件中 注释
///
fn parse_comment(
  options: &ParseOption,
  origin_charlist: &Vec<char>,
  locmap: &Option<LocMap>,
) -> Result<Vec<CommentNode>, String> {
  let mut blocklist: Vec<CommentNode> = vec![];
  let mut commentlist: Vec<String> = vec![];

  // 是否在 注释 存入中
  let mut wirte_comment = false;
  let mut wirte_line_comment = false;
  let mut wirte_closure_comment = false;

  // 块等级
  let mut braces_level = 0;

  // 结束标记 & 开始标记
  let start_braces = '{';
  let end_braces = '}';
  // 注释的内容共
  let comment_flag = "//".to_string();
  let comment_mark_strat = "/*".to_string();
  let comment_mark_end = "*/".to_string();

  // 如果启用 sourcemap 则用来记录坐标
  let mut record_loc: Option<Loc> = None;

  let mut index = 0;
  let mut start_index: Option<usize> = None;
  while index < origin_charlist.len() {
    // 处理字符
    let char = origin_charlist.get(index).unwrap().clone();
    // 优先检测注释 与当前块 等级 相同 为 0
    let word = origin_charlist.try_getword(index, 2).unwrap();
    if word == comment_flag && braces_level == 0 && !wirte_comment {
      wirte_comment = true;
      wirte_line_comment = true;
    } else if word == comment_mark_strat && braces_level == 0 && !wirte_comment {
      wirte_comment = true;
      wirte_closure_comment = true;
    }
    if braces_level == 0
      && wirte_comment
      && ((wirte_line_comment && (char == '\n' || char == '\r'))
        || (wirte_closure_comment && word == comment_mark_end))
    {
      wirte_comment = false;
      if wirte_line_comment {
        index += 1;
        commentlist.push(char.to_string());
        wirte_line_comment = false;
      } else if wirte_closure_comment {
        index += 2;
        commentlist.push(word.clone());
        wirte_closure_comment = false;
      }
      let comment = CommentNode {
        content: commentlist.join(""),
        loc: record_loc,
        startindex: start_index.unwrap(),
      };
      blocklist.push(comment);
      commentlist.clear();
      start_index = None;
      record_loc = None;
      continue;
    }
    if wirte_comment {
      // 如果启用 sourcemap 则记录坐标
      if options.sourcemap && char != '\r' && char != '\n' && record_loc.is_none() {
        record_loc = Some(locmap.as_ref().unwrap().get(&index).unwrap());
      }
      if start_index.is_none() {
        start_index = Some(index);
      }
      commentlist.push(char.to_string());
    }
    // ignore 忽略 大括号区域
    if char == start_braces {
      braces_level += 1;
    }
    if char == end_braces {
      braces_level -= 1;
    }
    index += 1;
  }

  if braces_level != 0 {
    return Err("the content contains braces that are not closed!".to_string());
  }
  Ok(blocklist)
}

///
/// 从当中的 成熟 AST 中获取 注释节点
///
fn get_comment_blocknode(block_node: &[StyleNode]) -> Vec<CommentNode> {
  let mut list = vec![];
  block_node.iter().for_each(|x| {
    if let StyleNode::Comment(cc) = x.deref().clone() {
      list.push(cc);
    }
  });
  list
}

///
/// 移除注释
/// 必须依赖开启 sourcemap
///
fn rm_comment(commentlist: &[CommentNode], origin_charlist: &Vec<char>) -> String {
  return if commentlist.is_empty() {
    origin_charlist.iter().collect::<String>()
  } else {
    let mut charlist = origin_charlist.to_owned();
    for cc in commentlist {
      let length = cc.content.len();
      let start = cc.startindex;
      let end = cc.startindex + length;
      let mut i = start;
      while i < end {
        let char = charlist.get(i).unwrap();
        if *char != '\n' && *char != '\r' {
          charlist[i] = char::from(32);
        }
        i += 1;
      }
    }
    charlist.iter().collect::<String>()
  };
}

///
/// 是否跳过 返回结果 [0]
/// 跳过索引 返回结果 [1]
///
pub fn skip_comment() -> Box<dyn FnMut(String, char, &mut usize) -> bool> {
  let comment_flag = "//".to_string();
  let comment_mark_strat = "/*".to_string();
  let comment_mark_end = "*/".to_string();
  let mut comment_inline = false;
  let mut comment_mark = false;
  Box::new(move |word, char, index| {
    if word == comment_flag && !comment_inline {
      comment_inline = true;
    }
    if word == comment_mark_strat && !comment_mark {
      comment_mark = true;
    }
    if (char == '\n' || char == '\r') && comment_inline {
      comment_inline = false;
    }
    if word == comment_mark_end && comment_mark {
      comment_mark = false;
      *index += 1;
    }
    comment_inline || comment_mark
  })
}
