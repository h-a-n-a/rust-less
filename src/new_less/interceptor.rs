use crate::new_less::file::path_resolve;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

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
      let cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
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
      let js_file = path_resolve("./dist/main.js");
      let mut task = Command::new("npx");
      task.arg("ts-node");
      task.arg(js_file);
      task.arg("--content");
      task.arg(serde_json::to_string(&content_map).unwrap());
      task.arg("--option");
      task.arg(serde_json::to_string(&option_map).unwrap());

      task.current_dir(cwd);
      task.stdout(Stdio::piped());

      let task_res = task
        .output()
        .expect(format!("{}->less.js callback is failed", filepath).as_str());
      let status = task_res.status.code().unwrap();
      let content = std::str::from_utf8(&*task_res.stdout).unwrap().to_string();
      return if status > 0 {
        Err(format!(
          "parse less file ->{} \n has error in interceptor,\n ex is \n {}",
          filepath, content
        ))
      } else {
        Ok(content)
      };
      // let content = "";
    }
    Ok(content.to_string())
  }
}
