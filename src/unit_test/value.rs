use crate::new_less::value::ValueNode;

#[test]
fn test_value_parse() {
  let vars_list = vec![
    r#"@width"#.to_string(),
    r#"@abc-bcd"#.to_string(),
    r#"@abc - @bcd"#.to_string(),
  ];
  let mut haserror = 0;
  vars_list.into_iter().for_each(|tt| {
    match ValueNode::new(tt, None) {
      Ok(obj) => {
        println!("{:#?}", obj);
      }
      Err(msg) => {
        haserror += 1;
        println!("{}", msg);
      }
    }
  });
  assert_eq!(haserror, 0);
}