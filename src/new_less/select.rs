use crate::extend::string::StringExtend;

pub struct Selector {
  origin_txt: String,
  rule: Vec<SelectorRule>,
}

pub struct SelectorRule {
  origin_txt: String,
}


impl Selector {
  ///
  /// 初始化方法
  ///
  fn new(txt: String) -> Selector {
    Selector {
      origin_txt: txt,
      rule: vec![],
    }
  }
  
  
  fn analysis(&self) {
    let class_selector = ".";
    let id_selector = "#";
    let borther_selector = "~";
    let space = " ";
    let new_line_v1 = "\n";
    let new_line_v2 = "\r";
    let column_combinator = "||";
    let pseudo = ":";
    let attr_bgein = "[";
    let attr_end = "]";
    
    let var_char = "@";
    let comment_char = "/";
    let add_char = "+";
    let normal_char = "*";
    let sub_char = "-";
    let line_char = "_";
    
    let end_queto = ";";
    let quota_mark = "'";
    let double_quota_mark = r#"""#;
    
    let brackets_start = "(";
    let brackets_end = ")";
    
    
    
    let charlist = self.origin_txt.tocharlist();
  }
}