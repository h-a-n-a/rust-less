use crate::new_less::block::{OriginBlock, OriginBlockType};
use crate::new_less::loc::Loc;

pub fn parse_origin_block(content: String) -> Result<Vec<OriginBlock>, String> {
  let charlist = content.chars().map(|x| x.to_string()).collect::<Vec<String>>();
  let mut blocklist: Vec<OriginBlock> = vec![];
  let mut templist: Vec<String> = vec![];
  let mut commentlist: Vec<String> = vec![];
  let mut index = 0;
  
  // 是否在 注释 存入中
  let mut wirte_comment = false;
  let mut wirte_line_comment = false;
  let mut wirte_closure_comment = false;
  // 块等级
  let mut braces_level = 0;
  
  // 结束标记 & 开始标记
  let endqueto = ";".to_string();
  let start_braces = "{".to_string();
  let end_braces = "}".to_string();
  // 注释的内容共
  let comment_flag = "//".to_string();
  let comment_mark_strat = "/*".to_string();
  let comment_mark_end = "*/".to_string();
  
  let mut record_loc = false;
  let mut col = 0;
  let mut column: Vec<usize> = vec![];
  let mut loc: Loc = Loc {
    line: 0,
    col: 0,
  };
  
  while index < charlist.len() {
    
    // 处理字符
    let prev_char;
    if index != 0 {
      prev_char = charlist.get(index - 1).unwrap().clone();
    } else {
      prev_char = "".to_string();
    }
    let mut char = charlist.get(index).unwrap().clone();
    let next_char;
    if index != charlist.len() - 1 {
      next_char = charlist.get(index + 1).unwrap().clone();
    } else {
      next_char = "".to_string()
    }
    // 处理坐标
    if char != '\n'.to_string() && char != '\r'.to_string() && !record_loc {
      loc = Loc {
        line: column.len(),
        col: col.clone(),
      };
      record_loc = true;
    }
    
    if char == '\n'.to_string() || char == '\r'.to_string() {
      column.push(col);
      col = 0;
    } else {
      col += 1;
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
        loc: loc.clone(),
      });
      record_loc = false;
      commentlist.clear();
      continue;
    }
    if !wirte_comment {
      templist.push(char.clone());
    } else {
      commentlist.push(char.clone());
      index += 1;
      continue;
    }
    // 进入 style_list 中 块级内容 延迟后续进行 -> 递归计算
    if char == start_braces {
      braces_level += 1;
    }
    if char == end_braces {
      braces_level -= 1;
      if braces_level == 0 {
        blocklist.push(OriginBlock {
          block_type: OriginBlockType::Style_Rule,
          content: templist.join(""),
          loc: loc.clone(),
        });
        templist.clear();
        record_loc = false;
      }
    }
    // style_list 外部的内容 进行 变量 | 引用 | 注释 的标准计算
    if char == endqueto && braces_level == 0 {
      blocklist.push(OriginBlock {
        block_type: OriginBlockType::Var,
        content: templist.join(""),
        loc: loc.clone(),
      });
      templist.clear();
      record_loc = false;
    }
    index += 1;
  }
  
  if braces_level != 0 {
    return Err("the content contains braces that are not closed!".to_string());
  }
  
  Ok(blocklist)
}