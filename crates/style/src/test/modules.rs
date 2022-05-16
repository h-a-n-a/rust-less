use crate::extend::string::StringExtend;
use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;

#[test]
fn test_less_render() {
  let filepath = path_resolve("assets/css_modules/index.module.less");
  let app = Application::default();
  {
    app.context.lock().unwrap().option.hooks.content_interceptor = None;
  }
  let res = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.x {
  display: inline-block;
  width: 20px;
}

h2,h3 {
  font-size: 10px;
  display: block;
}

h2 .a_css_modules_index_module_34d8f9473ef78d378b2faca28ff125b7dea380d9,h3 .a_css_modules_index_module_34d8f9473ef78d378b2faca28ff125b7dea380d9 {
  display: block;
  box-sizing: border-box;
}

h2 .m_css_modules_index_module_34d8f9473ef78d378b2faca28ff125b7dea380d9 .tap #h2,h3 .m_css_modules_index_module_34d8f9473ef78d378b2faca28ff125b7dea380d9 .tap #h2 {
  word-break: break-all;
  width: 40px;
}

.kol_css_modules_index_module_34d8f9473ef78d378b2faca28ff125b7dea380d9 h2 .m_css_modules_index_module_34d8f9473ef78d378b2faca28ff125b7dea380d9 .tap #h2,.kol_css_modules_index_module_34d8f9473ef78d378b2faca28ff125b7dea380d9 h3 .m_css_modules_index_module_34d8f9473ef78d378b2faca28ff125b7dea380d9 .tap #h2 {
  width: 100px;
}

.u_css_modules_index_module_34d8f9473ef78d378b2faca28ff125b7dea380d9 h2,.u_css_modules_index_module_34d8f9473ef78d378b2faca28ff125b7dea380d9 h3 {
  display: inline-block;
  width: 20px;
}

h2 .b,h3 .b {
  display: inline-block;
  width: 20px;
}

.c h2,.c h3 {
  display: inline-block;
  width: 20px;
}
  "#;
  println!("{}", res);
  assert_eq!(target_code.to_string().simple_compare(), res.simple_compare());
}