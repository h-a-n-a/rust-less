#[cfg(test)]
mod tests {
  use crate::extend::string::StringExtend;
  use crate::extend::time::wastetime;
  use crate::extend::vec_str::VecStrExtend;
  use crate::new_less::block::OriginBlock;
  use crate::new_less::comment::{Comment, skip_comment};
  use crate::new_less::file::*;
  use crate::new_less::file_manger::FileManger;
  use crate::new_less::fileinfo::*;
  use crate::new_less::loc::LocMap;
  
  #[test]
  fn test_less() {
    let start_record = wastetime("test_less");
    // 处理过程
    let filepath = path_resolve("assets/demo.less");
    // let info = FileInfo::create_disklocation(filepath, Default::default()).unwrap();
    let mut index = 0;
    while index < 100 {
      FileInfo::create_disklocation(filepath.clone(), Default::default()).unwrap();
      index += 1;
    }
    start_record();
    // let json = serde_json::to_string_pretty(&info).unwrap();
    // println!("{}", json);
    println!("........");
  }
  
  ///
  /// 测试字典方法
  ///
  #[test]
  fn test_loc() {
    let content = readfile(path_resolve("assets/demo.less")).unwrap();
    let obj = LocMap::new(content);
    let c = obj.get(0).unwrap();
    let x = obj.getloc(17, 25).unwrap();
    assert_eq!(c.char, "@".to_string());
    assert_eq!(x.char, ";".to_string());
  }
  
  #[test]
  fn test_rel_path() {
    let a = "../test/a.txt".to_string();
    let b = "./test/a.txt".to_string();
    let c = "/test/a.txt".to_string();
    assert_eq!(FileManger::is_relative_path(&a), true);
    assert_eq!(FileManger::is_relative_path(&b), true);
    assert_eq!(FileManger::is_relative_path(&c), true);
  }
  
  #[test]
  fn test_comment_remove() {
    let start_record = wastetime("test_less");
    let filepath = path_resolve("assets/demo.less");
    let info = FileInfo::create_disklocation(filepath, Default::default()).unwrap();
    let content = info.rm_comment();
    start_record();
    let target = r#"
@width:400px;
@height:300px;
@font_size:12px;
textarea {
  width:@width;
  height:@height;
  font-size:@font_size;
  @font_size:12px;
  .a{
    font-size:@font_size;

    .c{
      font-size:@font_size;
    }
  }
  .b{
    font-size:@font_size;
  }
}
a{
  font-size:@font_size;

  .c{
    font-size:@font_size;
  }
}
    "#;
    assert_eq!(content.simple_compare(), target.to_string().simple_compare());
  }
  
  #[test]
  fn test_skip_comment() {
    let start_record = wastetime("test_less");
    // 处理过程
    let filepath = path_resolve("assets/demo.less");
    let conetnt = readfile(filepath).unwrap().tocharlist();
    let mut i = 0;
    let mut skipcall = skip_comment();
    while i < conetnt.len() {
      let word = conetnt.try_getword(i, 2).unwrap();
      let char_val = conetnt.get(i).unwrap().to_string();
      let old_i = i;
      let skip_res = skipcall(word, char_val.clone(), &mut i);
      if !skip_res && old_i == i {
        print!("{}", char_val);
      }
      i += 1;
    }
    start_record();
    println!("........");
  }
  
  
  #[test]
  fn test_error_var_check() {
    let code = r#"
@width:400px;
a{
  font-size:10px;

  .c{
    font-size:20px;
  }
}

dfkljaskdlfjadfjadlskfj

asldkfjak
    "#;
    let msg = FileInfo::create_txt_content(code.to_string(), Default::default(), None).err().unwrap();
    assert_eq!(msg.simple_compare(), "the word is not with endqueto -> dfkljaskdlfjadfjadlskfjasldkfjak".to_string().simple_compare());
    println!("........");
  }
}