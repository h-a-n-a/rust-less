use std::rc::Rc;
use crate::extend::string::StringExtend;
use crate::new_less::block::OriginBlock;
use crate::new_less::file_manger::FileManger;
use crate::new_less::loc::LocMap;
use crate::new_less::option::ParseOption;
use crate::new_less::origin_parse::parse_origin_block;

#[derive(Debug, Clone)]
pub struct FileInfo {
  // 文件的磁盘位置
  pub disk_location: Option<std::string::String>,
  // 文件的原始内容
  pub origin_txt_content: String,
  // 根据 原始内容 -> 转化的 字符数组
  pub origin_charlist: Vec<String>,
  // 文件的 原始AST节点
  pub block_node: Vec<OriginBlock>,
  // 当前所有 索引 对应的 坐标行列 -> 用于执行 sourcemap
  pub locmap: Option<LocMap>,
  // 内部调用方式时 需要拿到对应的 转化配置
  option: ParseOption,
  // 当前引用链
  pub import_file: Vec<Rc<FileInfo>>,
  // 所有引用链
  pub recur_import_file: Vec<Rc<FileInfo>>,
}


impl FileInfo {
  pub fn get_options(&self) -> &ParseOption {
    &self.option
  }

  pub fn get_loc_by_content(content: &str) -> LocMap {
    let locmap = LocMap::new(content.to_string());
    locmap
  }

  pub fn get_charlist(content: &str) -> Vec<String> {
    content.to_string().tocharlist()
  }

  ///
  /// 根据文件路径 解析 文件
  ///
  pub fn create_disklocation(filepath: String, option: ParseOption) -> Result<FileInfo, String> {
    let abs_path: String;
    let text_content: String;
    let charlist: Vec<String>;
    let mut locmap: Option<LocMap> = None;
    let block_node;
    match FileManger::resolve(
      filepath.clone(),
      option.include_path.clone()) {
      Ok((calc_path, content)) => {
        abs_path = calc_path;
        text_content = content.clone();
        if option.sourcemap {
          locmap = Some(FileInfo::get_loc_by_content(content.as_str()));
        }
        charlist = FileInfo::get_charlist(content.as_str());
        match parse_origin_block(content) {
          Ok(blocks) => {
            block_node = blocks;
          }
          Err(msg) => {
            return Err(msg);
          }
        }
      }
      Err(msg) => {
        return Err(msg);
      }
    }
    let obj = FileInfo {
      disk_location: Some(abs_path),
      block_node,
      origin_txt_content: text_content,
      origin_charlist: charlist,
      locmap,
      option,
      import_file: vec![],
      recur_import_file: vec![],
    };
    Ok(obj)
  }
}





