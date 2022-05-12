use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::{Command, Stdio};
use crate::new_less::file::path_resolve;

pub struct LessInterceptor;


impl LessInterceptor {
  pub fn handle(filepath: &str, content: &str) -> Result<String, String> {
    let path = Path::new(filepath);
    if path.extension() == Some("less".to_string().as_ref()) {
      let self_dir = Path::new(filepath).parent().unwrap().to_str().unwrap().to_string();
      let word_cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
      let include_path = vec![self_dir, word_cwd, "node_modules".to_string()];
      let mut option_map = HashMap::new();
      option_map.insert("paths", include_path);
      let mut content_map = HashMap::new();
      content_map.insert("content".to_string(), content.to_string());
      let mut task = Command::new("node");
      task.arg("./dist/main.js");
      task.arg("--content");
      task.arg(serde_json::to_string(&content_map).unwrap());
      task.arg("--option");
      task.arg(serde_json::to_string(&option_map).unwrap());
      let rs_lib_dir = path_resolve("");
      task.current_dir(rs_lib_dir);
      task.stdout(Stdio::piped());

      let mut task = task.spawn().expect(format!("{}->less.js callback is failed", filepath).as_str());
      let status = task.wait().unwrap();
      let output = task
        .wait_with_output()
        .expect("failed to wait on child");
      let content = std::str::from_utf8(&*output.stdout).unwrap().to_string();
      return if status.code().unwrap() > 0 {
        Err(format!("parse less file ->{} \n has error in interceptor,\n ex is \n {}", filepath, content))
      } else {
        Ok(content)
      }
    }
    Ok(content.to_string())
  }
}