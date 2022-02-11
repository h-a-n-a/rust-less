use crate::new_less::file::{cmd_path, cmd_path_resolve, path_join, readfile};
use std::path::Path;

pub struct FileManger {}

impl FileManger {
  ///
  /// 文件查找对应解析路径
  ///
  pub fn resolve(
    filepath: String,
    include_path: Option<Vec<String>>,
  ) -> Result<(String, String), String> {
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
      if let Some(mut paths) = include_path {
        paths.insert(0, cmd_path());
        let mut abs_path: Option<String> = None;
        let mut failpath = vec![];
        for basepath in paths {
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
        let abs_path = cmd_path_resolve(filepath.as_str());
        let path_target = Path::new(abs_path.as_str());
        match checkpath(path_target) {
          Ok(_) => {}
          Err(msg) => {
            return Err(msg);
          }
        }
        Ok((abs_path.clone(), readfile(abs_path).unwrap()))
      }
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
    let mut matched = false;
    if (!txt.is_empty() && &txt[0..1] == "/")
      || (txt.len() >= 2 && &txt[0..2] == "./")
      || (txt.len() >= 3 && &txt[0..3] == "../")
    {
      matched = true
    }
    matched
  }
}
