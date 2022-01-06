#[cfg(test)]
mod tests {
  use std::ffi::OsString;
  use std::path::Path;
  use crate::extend::string::StringExtend;
  
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
  
  fn path_resolve(path: &str) -> String {
    let work_cwd = env!("CARGO_MANIFEST_DIR");
    let os_work_cwd = OsString::from(work_cwd);
    return Path::new(&os_work_cwd)
      .join(path)
      .into_os_string()
      .into_string()
      .unwrap();
  }
  
  fn readfile(path: String) -> Option<String> {
    let filepath = Path::new(&path);
    if filepath.exists() {
      match std::fs::read_to_string(filepath) {
        Ok(content) => { Some(content) }
        Err(_) => { None }
      }
    } else {
      None
    }
  }
  
  #[test]
  fn test_str() {
    let strore = "123456";
    let a = &strore[0..2];
    let b = &strore[1..3];
    let index_1 = strore.to_string().indexOf("23", Some(2));
    let index_2 = strore.to_string().indexOf("23", Some(1));
    let index_3 = strore.to_string().indexOf("23", None);
    println!("......");
    assert_eq!(index_1, -1);
    assert_eq!(index_2, 1);
    assert_eq!(index_3, 1);
    let t = &strore[1..1];
    println!("......");
  }
  
  #[derive(Debug, Clone)]
  enum OriginBlockType {
    comment,
    style_rule,
    var,
    import,
  }
  
  #[derive(Debug, Clone)]
  struct Loc {
    line: usize,
    col: usize,
  }
  
  #[derive(Debug, Clone)]
  struct OriginBlock {
    block_type: OriginBlockType,
    content: String,
    loc: Loc,
  }
  
  #[test]
  fn test_less() {
    let content = readfile(path_resolve("assets/demo.less")).unwrap();
    
    /// 转化 Less 文件中 最原始的 Block 片段
    fn ParseOriginBlock(content: String) -> Result<Vec<OriginBlock>, String> {
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
            block_type: OriginBlockType::comment,
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
              block_type: OriginBlockType::style_rule,
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
            block_type: OriginBlockType::var,
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
    
    let blocks = ParseOriginBlock(content).unwrap();
    println!("{:#?}", blocks);
    println!("........");
  }
}
