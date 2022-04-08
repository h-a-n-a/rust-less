use crate::extend::string::StringExtend;
use crate::new_less::context::Context;
use crate::new_less::file::path_resolve;

#[test]
fn test_arco_pro_less_render() {
  // let white_list = vec![2, 3, 4, 5, 6];
  let white_list = vec![5];
  let mut index = 1;
  while index < 44 {
    if white_list.contains(&index) {
      let filepath = path_resolve(format!("assets/arco-pro/{}.less", index).as_str());
      let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
      let css = context.render(filepath).unwrap();
      println!("{:#?}", css);
    }
    index += 1;
  }
}

#[test]
fn test_arco_pro_2_less() {
  let filepath = path_resolve("assets/arco-pro/2.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.footer{
  display: flex;
  align-items: center;
  justify-content: center;
  height: 40px;
  text-align: center;
  color: var(--color-text-2);
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_3_less() {
  let filepath = path_resolve("assets/arco-pro/3.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.message-box{
  width: 400px;
  max-height: 800px;
  background-color: var(--color-bg-popup);
  border: 1px solid var(--color-border-2);
  box-shadow: 0 4px 10px rgba(0 , 0 , 0 , 0.1);
  border-radius: 4px;

}

.message-box :global(.arco-tabs-header-nav){
  padding: 8px 16px;
  border-bottom: 1px solid var(--color-border-2);

}

.message-box :global(.arco-list-item-meta){
  align-items: flex-start;

}

.message-box :global(.arco-list-item-meta-content){
  width: 100%;

}

.message-box :global(.arco-tabs-content){
  padding-top: 0;

}

.message-title{
  display: flex;
  justify-content: space-between;

}

.footer{
  display: flex;

}

.footer-item{
  display: flex;
  justify-content: center;
  width: 50%;

}

.footer-item:first-child{
  border-right: 1px solid var(--color-border-2);

}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_4_less() {
  let filepath = path_resolve("assets/arco-pro/4.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.icon-button{
  font-size: 16px;
  border: 1px solid var(--color-border-2);
}

.icon-button > svg{
  vertical-align: -3px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_5_less() {
  let filepath = path_resolve("assets/arco-pro/5.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.icon-button{
  font-size: 16px;
  border: 1px solid var(--color-border-2);
}

.icon-button > svg{
  vertical-align: -3px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_arco_pro_6_less() {
  let filepath = path_resolve("assets/arco-pro/6.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let css = context.render(filepath).unwrap();
  let target_code = r#"
.panel{
  background-color: var(--color-bg-2);
  border-radius: 4px;
}
  "#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}
