use crate::extend::string::StringExtend;
use crate::extend::time::wastetime;
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::comment::{skip_comment, Comment};
use crate::new_less::file::{path_resolve, readfile};
use crate::new_less::fileinfo::FileInfo;

#[test]
fn test_comment_remove() {
  let record = wastetime("test_less");
  let filepath = path_resolve("assets/demo.less");
  let info = FileInfo::create_disklocation(filepath, Default::default()).unwrap();
  let content = info.rm_comment();
  record();
  let target = r#"
@width: 400px;
@height: 300px;
@font_size: 12px;

textarea {
  width: @width;
  height: @height;
  font-size: @font_size;
  @font_size: 12px;

  .a {
    font-size: @font_size;

    .c {
      font-size: @font_size;
    }
  }

  .b {
    font-size: @font_size;
  }
}


a {
  font-size: @font_size;

  @media screen and ( max-width: 900px) {
    font-size: @font_size;
  }
}
    "#;
  assert_eq!(
    content.simple_compare(),
    target.to_string().simple_compare()
  );
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
