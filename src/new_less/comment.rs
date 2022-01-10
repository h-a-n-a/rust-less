use crate::new_less::block::{OriginBlock, OriginBlockType};
use crate::new_less::fileinfo::FileInfo;
use crate::new_less::loc::Loc;

pub trait Comment {
  fn get_comment(&self) -> Result<Vec<OriginBlock>, String>;
  fn get_comment_blocknode(&self) -> Vec<OriginBlock>;
  fn rm_comment(&self) -> String;
}

impl Comment for FileInfo {
  ///
  /// 获取一段 文件中 注释
  ///
  fn get_comment(&self) -> Result<Vec<OriginBlock>, String> {
    let options = self.get_options();
    let mut blocklist: Vec<OriginBlock> = vec![];
    let mut commentlist: Vec<String> = vec![];
    
    // 是否在 注释 存入中
    let mut wirte_comment = false;
    let mut wirte_line_comment = false;
    let mut wirte_closure_comment = false;
    
    // 块等级
    let mut braces_level = 0;
    
    // 结束标记 & 开始标记
    let start_braces = "{".to_string();
    let end_braces = "}".to_string();
    // 注释的内容共
    let comment_flag = "//".to_string();
    let comment_mark_strat = "/*".to_string();
    let comment_mark_end = "*/".to_string();
    
    // 如果启用 sourcemap 则用来记录坐标
    let mut record_loc: Option<Loc> = None;
    
    let mut index = 0;
    while index < self.origin_charlist.len() {
      // 处理字符
      let char = self.origin_charlist.get(index).unwrap().clone();
      let next_char;
      if index != self.origin_charlist.len() - 1 {
        next_char = self.origin_charlist.get(index + 1).unwrap().clone();
      } else {
        next_char = "".to_string()
      }
      
      // 优先检测注释 与当前块 等级 相同 为 0
      let word = char.clone() + &next_char;
      if word == comment_flag && braces_level == 0 && !wirte_comment {
        wirte_comment = true;
        wirte_line_comment = true;
      } else if word == comment_mark_strat && braces_level == 0 && !wirte_comment {
        wirte_comment = true;
        wirte_closure_comment = true;
      }
      
      if braces_level == 0 &&
        wirte_comment &&
        (
          (wirte_line_comment && (&char == "\n" || &char == "\r")) ||
            (wirte_closure_comment && word == comment_mark_end)
        ) {
        wirte_comment = false;
        if wirte_line_comment {
          index += 1;
          commentlist.push(char.clone());
          wirte_line_comment = false;
        } else if wirte_closure_comment {
          index += 2;
          commentlist.push(word.clone());
          wirte_closure_comment = false;
        }
        blocklist.push(OriginBlock {
          block_type: OriginBlockType::Comment,
          content: commentlist.join(""),
          loc: record_loc.unwrap(),
        });
        commentlist.clear();
        record_loc = None;
        continue;
      }
      if wirte_comment {
        // 如果启用 sourcemap 则记录坐标
        if options.sourcemap && char != "\r" && char != "\n" && record_loc.is_none() {
          record_loc = Some(self.locmap.as_ref().unwrap().get(index).unwrap());
        }
        commentlist.push(char.clone());
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
  fn get_comment_blocknode(&self) -> Vec<OriginBlock> {
    let blocknode_ref = &self.block_node;
    blocknode_ref
      .into_iter()
      .filter(|&node| node.block_type == OriginBlockType::Comment)
      .map(|c| c.clone())
      .collect::<Vec<OriginBlock>>()
  }
  
  ///
  /// 移除注释
  ///
  fn rm_comment(&self) -> String {
    let commentlist = self.get_comment_blocknode();
    return if commentlist.is_empty() {
      self.origin_txt_content.clone()
    } else {
      let mut charlist = self.origin_charlist.clone();
      for cc in commentlist {
        let length = cc.content.len();
        let start = cc.loc.index;
        let end = cc.loc.index + length;
        let mut i = start;
        while i < end {
          let char = charlist.get(i).unwrap();
          if char != "\n" && char != "\r" {
            charlist[i] = " ".to_string();
          }
          i += 1;
        }
      }
      charlist.join("")
    };
  }
}