use crate::extend::string::StringExtend;
use crate::new_less::context::Context;
use crate::new_less::file::path_resolve;

#[test]
fn test_arco_pro_less_render() {
  let white_list = vec![2];
  let mut index = 1;
  while index < 44 {
    if white_list.contains(&index) {
      let filepath = path_resolve(format!("assets/arco-pro/{}.less", index).as_str());
      let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
      let css = context.render(filepath).unwrap();
      println!("{}", css);
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
