use iocutil::prelude::{ContentHash, Hasher};
use std::path::Path;

pub struct StyleHash;

impl StyleHash {
  ///
  /// 根据文章内容生成 -> content_hash
  ///
  pub fn generate_hash_by_content(content: &str) -> String {
    let mut reader: &[u8] = content.as_bytes();
    let mut hasher = Hasher::new();
    std::io::copy(&mut reader, &mut hasher).unwrap();
    let c: ContentHash = hasher.digests();
    c.sha1.to_string()
  }

  ///
  /// 根据 css 文件路径 和 文件内容 生成对应 css_modules 的前缀
  ///
  pub fn generate_css_module_hash(abs_filepath: &str, content: &str) -> String {
    let path = Path::new(abs_filepath);
    let mut perfix = "".to_string();
    if let Some(parent_path) = path.parent() {
      perfix += parent_path.file_name().unwrap().to_str().unwrap();
      perfix += "_";
    }
    perfix += path.file_stem().unwrap().to_str().unwrap().replace(".", "_").as_str();
    let content_hash = Self::generate_hash_by_content(content);
    format!("{}_{}", perfix, content_hash)
  }
}
