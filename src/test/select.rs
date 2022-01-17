use crate::new_less::select::Selector;

#[test]
fn test_select_parse() {
  let demo_select_txt = r#".a .b"#.to_string();
  let ss = Selector::new(demo_select_txt);
}