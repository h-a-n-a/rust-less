use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio};
use crate::new_less::file::path_resolve;

pub struct LessInterceptor;

impl LessInterceptor {
  pub fn handle(filepath: &str, content: &str) -> Result<String, String> {
    let path = Path::new(filepath);
    if path.extension() == Some("less".to_string().as_ref()) {
      let self_dir = Path::new(filepath)
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
      let cwd = path_resolve("");
      let include_path = vec![
        Value::String(self_dir),
        Value::String(cwd.clone()),
        Value::String("node_modules".to_string()),
      ];
      let mut option_map = HashMap::new();
      option_map.insert("paths", Value::Array(include_path));
      option_map.insert("filename", Value::String(filepath.to_string()));
      let mut content_map = HashMap::new();
      content_map.insert("content".to_string(), content.to_string());
      let content_arg = serde_json::to_string(&content_map).unwrap();
      let option_arg = serde_json::to_string(&option_map).unwrap();
      let js_file = path_resolve("js-lib/dist/main.js");
      let mut task = Command::new("node");
      task.arg(js_file.as_str());
      task.arg("--content");
      task.arg(content_arg.as_str());
      task.arg("--option");
      task.arg(option_arg.as_str());
      task.current_dir(cwd);
      task.stdout(Stdio::piped());

      let _test_node_content = format!("node {} --content {} --option {}", js_file, content_arg, option_arg);

      let output = task
        .output()
        .expect(format!("{}->less.js callback is failed", filepath).as_str());
      let status = output.status.code().unwrap();
      let less_content = std::str::from_utf8(&*output.stdout).unwrap().to_string();
      return if status > 0 {
        Err(format!(
          "parse less file ->{} \n has error in interceptor,\n ex is \n {}",
          filepath, less_content
        ))
      } else {
        Ok(less_content)
      };
      // let content = "";
    }
    Ok(content.to_string())
  }
}
