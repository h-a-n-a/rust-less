use crate::extend::string::StringExtend;
use crate::new_less::new_select::{NewSelector, Paradigm};
use crate::new_less::var::HandleResult;

#[test]
fn test_select_parse() {

  let demo_select_list = vec![
    r#".a .b"#.to_string(),
    r#".a>.b"#.to_string(),
    r#"h1>.b"#.to_string(),
    r#"h1>#b1"#.to_string(),
    r#"h1~#b"#.to_string(),
    r#"h1~textarea"#.to_string(),
    r#"h1~*textarea"#.to_string(),
    r#"h1~img"#.to_string(),
    r#"*h1~*textarea"#.to_string(),
    r#".a.b"#.to_string(),
    r#"*.a+*.b"#.to_string(),
    r#">a"#.to_string(),
    r#">.b"#.to_string(),
    r#".b > a"#.to_string(),
    r#"p::first-line"#.to_string(),
    r#"selector:pseudo-class"#.to_string(),
  ];

  let target = r#"
.a .b
.a>.b
h1>.b
h1>#b1
h1~#b
h1~textarea
h1~*textarea
h1~img
*h1~*textarea
.a.b
*.a+*.b
>a
>.b
.b>a
p::first-line
selector:pseudo-class
  "#;


  let mut base = "".to_string();
  demo_select_list.into_iter().for_each(|tt| {
    let res = match NewSelector::new(tt.tocharlist(), None, None, None) {
      HandleResult::Success(obj) => Some(obj),
      HandleResult::Fail(msg) => {
        println!("{}", msg);
        None
      }
      HandleResult::Swtich => {
        println!("the type is not support select_txt!");
        None
      }
    };
    if res.is_none() {
      panic!("parse has error!");
    }
    let ss = res.unwrap();
    let value;
    if ss.paradigm_vec.len() < 2 {
      value = ss.paradigm_vec
        .iter()
        .map(|x|x.tostr())
        .collect::<Vec<String>>()
        .join("");
    } else {
      value = ss.paradigm_vec
        .iter()
        .map(|x|x.tostr())
        .collect::<Vec<String>>()
        .join(" -> ");
    }
    base += &value;
    println!("{:?}", value);
  });
  assert_eq!(base.simple_compare(), target.to_string().simple_compare());
}

#[test]
fn test_select_error_parse() {

}