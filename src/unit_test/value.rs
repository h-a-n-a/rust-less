use crate::extend::string::StringExtend;
use crate::new_less::value::ValueNode;

#[test]
fn test_value_parse() {
  let vars_list = vec![
    r#"@width;"#.to_string(),
    r#"@abc-bcd;"#.to_string(),
    r#"@abc - @bcd;"#.to_string(),
    r#"10px;"#.to_string(),
    r#"10px10 + 20px;"#.to_string(),
    r#""abc";"#.to_string(),
    r#"1px solid;"#.to_string(),
    r#".a >.b;"#.to_string(),
    r#"1px solid !important;"#.to_string(),
    r#"1px solid !abc;"#.to_string(),
    r#"1px solid #fff;"#.to_string(),
    r#"rgba(255, 255, 255, 0.12);"#.to_string(),
    r#"rgb(var(--warning-6), 0.35);"#.to_string(),
    r#"50%;"#.to_string(),
  ];
  let mut haserror = 0;
  vars_list
    .into_iter()
    .for_each(|tt| match ValueNode::new(tt.tocharlist(), None, None, None) {
      Ok(obj) => {
        println!("{:#?}", obj);
      }
      Err(msg) => {
        haserror += 1;
        println!("{}", msg);
      }
    });
  assert_eq!(haserror, 0);
}
