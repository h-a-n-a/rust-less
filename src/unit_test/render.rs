use crate::extend::string::StringExtend;
use crate::new_less::context::Context;
use crate::new_less::file::path_resolve;

#[test]
fn test_less_render() {
  let filepath = path_resolve("assets/test.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let res = context.render(filepath).unwrap();
  println!("{}", res);
}

#[test]
fn test_demo_render() {
  let filepath = path_resolve("assets/demo.less");
  let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  let res = context.render(filepath).unwrap();
  println!("{}", res);
  let target_code = r#"
  h2 {
  font-size: 10px;
  display: block;
}
h2 .a {
  display: block;
  box-sizing: border-box;
}

textarea {
  width: 400px;
  height: 300px;
  font-size: 12px;
  border: 601px solid #fff;
}
textarea .a {
  font-size: 12px;
}
textarea .a .c {
  font-size: 12px;
}
textarea .b {
  font-size: 12px;
}
.a {
  font-size: 12px;
}
@media screen and (max-width: 900px) {
  .a {
    font-size: 12px;
  }
}
@media screen and (min-width: 900px) {
  .xyz {
    font-size: 12px;
  }
}
@media screen and (min-width: 900px) and screen and (max-width: 900px) {
  .xyz {
    color: red;
  }
}
.ace {
  font-size: 10px;
}
.ace .b {
  font-size: 20px;
} 
  "#;
  assert_eq!(
    res.simple_compare(),
    target_code.to_string().simple_compare()
  );
}
