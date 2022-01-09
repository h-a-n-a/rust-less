use crate::extend::string::StringExtend;
use crate::new_less::fileinfo::FileInfo;

trait Comment{
  fn get_comment(&self);
}

impl Comment for FileInfo {
  fn get_comment(&self) {
    let charlist = self.origin_txt_content.tocharlist();
    let mut commentlist: Vec<String> = vec![];

    // 是否在 注释 存入中
    let mut wirte_comment = false;
    let mut wirte_line_comment = false;
    let mut wirte_closure_comment = false;
  }
}