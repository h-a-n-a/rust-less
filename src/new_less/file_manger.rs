use crate::new_less::file::{path_join, readfile};
use std::path::Path;

pub struct FileManger {}

impl FileManger {
  ///
  /// 文件查找对应解析路径
  ///
  pub fn resolve(filepath: String, include_path: Vec<String>) -> Result<(String, String), String> {
    // 检查文件是否 存在 闭包方法 被 下方 调用
    let checkpath = |path_target: &Path| -> Result<(), String> {
      if !path_target.exists() {
        return Err(format!("file is not exists filepath is {}", filepath));
      }
      if !path_target.is_file() {
        return Err(format!(
          "file is not file maybe is dir ?! filepath is{}",
          filepath
        ));
      }
      Ok(())
    };
    // 相对路径 和 绝对路径 分开计算
    return if FileManger::is_relative_path(&filepath) {
      // 相对路径的情况
      let mut abs_path: Option<String> = None;
      let mut failpath = vec![];
      for basepath in include_path {
        let temp_path = path_join(basepath.as_str(), filepath.as_str());
        let path_target = Path::new(temp_path.as_str());
        match checkpath(path_target) {
          Ok(_) => {
            abs_path = Some(temp_path.clone());
            break;
          }
          Err(_) => failpath.push(temp_path.clone()),
        }
      }
      return if let Some(match_path) = abs_path {
        Ok((match_path.clone(), readfile(match_path).unwrap()))
      } else {
        Err(format!(
          "Nothings File is find in cmdpath and inculdepath,{}",
          failpath.join(";")
        ))
      };
    } else {
      // 绝对路径的情况
      let path_target = Path::new(filepath.as_str());
      match checkpath(path_target) {
        Ok(_) => {}
        Err(msg) => {
          return Err(msg);
        }
      }
      let res = readfile(filepath.clone()).unwrap();
      Ok((filepath.clone(), res))
    };
  }

  pub fn is_relative_path(txt: &str) -> bool {
    let path = Path::new(txt);
    path.is_relative()
  }

  pub fn get_dir(path_value: &str) -> Result<String, String> {
    let path = Path::new(path_value);
    if path.is_file() {
      Ok(path.parent().unwrap().to_str().unwrap().to_string())
    } else if path.is_dir() {
      Ok(path_value.to_string())
    } else {
      Err(format!(
        "path type is file or dir please check {}",
        path_value
      ))
    }
  }
}
