use crate::new_less::select::Selector;

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
  ];
  demo_select_list.into_iter().for_each(|tt| {
    Selector::new(tt);
  });
}