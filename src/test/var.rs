use crate::extend::string::StringExtend;
use crate::new_less::fileinfo::FileInfo;
use crate::new_less::node::HandleResult;
use crate::new_less::var_node::VarNode;

#[test]
fn test_error_var_check() {
  let vars_list = vec![r#"@width:400px;"#.to_string()];
  let mut haserror = 0;
  vars_list
    .into_iter()
    .for_each(|tt| match VarNode::new(tt, None, None) {
      HandleResult::Success(obj) => {
        haserror += 0;
        println!("{:?}", obj);
      }
      HandleResult::Fail(msg) => {
        haserror += 1;
        println!("{:?}", msg);
      }
      HandleResult::Swtich => {
        haserror += 1;
        println!("{:?}", "swtich case ....");
      }
    });
  println!("........");
}
