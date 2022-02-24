use crate::new_less::context::Context;
use crate::new_less::node::HandleResult;
use crate::new_less::style_rule::StyleRuleNode;

#[test]
fn test_style_rule_parse() {
  let vars_list = vec![
    r#"box-sizing: border-box;"#.to_string(),
    r#"font-size: 10px;"#.to_string(),
  ];
  let mut haserror = 0;
  vars_list.into_iter().for_each(|tt| {
    match StyleRuleNode::new(tt, None, None, None, Context::default()) {
      HandleResult::Success(obj) => {
        haserror += 0;
        let json = serde_json::to_string_pretty(&obj).unwrap();
        println!("{}", json);
      }
      HandleResult::Fail(msg) => {
        haserror += 1;
        println!("{:?}", msg);
      }
      HandleResult::Swtich => {
        haserror += 1;
        println!("{:?}", "swtich case ....");
      }
    }
  });
  assert_eq!(haserror, 0);
}
