use crate::extend::string::StringExtend;
use crate::new_less::comment::Comment;
use crate::new_less::file::cmd_path_resolve;
use crate::new_less::file_manger::FileManger;
use crate::new_less::loc::LocMap;
use crate::new_less::node::{StyleNode, StyleNodeJson};
use crate::new_less::option::ParseOption;
use crate::new_less::rule::Rule;
use crate::new_less::var::Var;
use serde::Serialize;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub struct FileInfo {
  // 文件的磁盘位置
  pub disk_location: Option<std::string::String>,
  // 文件的原始内容
  pub origin_txt_content: String,
  // 根据 原始内容 -> 转化的 字符数组
  pub origin_charlist: Vec<String>,
  // 文件的 原始AST节点
  pub block_node: Vec<StyleNode>,
  // 当前所有 索引 对应的 坐标行列 -> 用于执行 sourcemap
  pub locmap: Option<LocMap>,
  // 内部调用方式时 需要拿到对应的 转化配置
  pub option: ParseOption,
  // 当前引用链
  pub import_file: Vec<Rc<RefCell<FileInfo>>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileInfoJson {
  pub disk_location: Option<std::string::String>,
  pub block_node: Vec<StyleNodeJson>,
  pub import_file: Vec<FileInfoJson>,
}

impl FileInfo {
  ///
  /// 转 json 标准化
  ///
  pub fn tojson(&self) -> FileInfoJson {
    let mut block_node = vec![];
    self
      .block_node
      .clone()
      .into_iter()
      .for_each(|node| match node {
        StyleNode::Comment(cc) => block_node.push(StyleNodeJson::Comment(cc)),
        StyleNode::Var(vv) => block_node.push(StyleNodeJson::Var(vv)),
        StyleNode::Rule(rule) => {
          let futex_rule = rule.deref().borrow().deref().clone().tojson();
          block_node.push(StyleNodeJson::Rule(futex_rule));
        }
      });
    let import_file = self
      .import_file
      .clone()
      .into_iter()
      .map(|x| x.borrow().tojson())
      .collect::<Vec<FileInfoJson>>();
    FileInfoJson {
      disk_location: self.disk_location.clone(),
      block_node,
      import_file,
    }
  }

  ///
  /// 转 heap 堆上对象
  ///
  pub fn toheap(self) -> Rc<RefCell<FileInfo>> {
    Rc::new(RefCell::new(self))
  }

  pub fn get_loc_by_content(content: &str) -> LocMap {
    LocMap::new(content.to_string())
  }

  ///
  /// 根据文件路径 解析 文件
  ///
  pub fn create_disklocation(filepath: String, option: ParseOption) -> Result<FileInfo, String> {
    let abs_path: String;
    let text_content: String;
    let charlist: Vec<String>;
    let mut locmap: Option<LocMap> = None;
    let mut obj: FileInfo;
    match FileManger::resolve(filepath, option.include_path.clone()) {
      Ok((calc_path, content)) => {
        abs_path = calc_path;
        text_content = content.clone();
        if option.sourcemap {
          locmap = Some(FileInfo::get_loc_by_content(content.as_str()));
        }
        charlist = content.tocharlist();
        obj = FileInfo {
          disk_location: Some(abs_path),
          block_node: vec![],
          origin_txt_content: text_content,
          origin_charlist: charlist,
          locmap,
          option,
          import_file: vec![],
        };
        match obj.parse() {
          Ok(_) => {}
          Err(msg) => {
            return Err(msg);
          }
        }
      }
      Err(msg) => {
        return Err(msg);
      }
    }
    Ok(obj)
  }

  ///
  /// 根据文件内容 解析文件
  ///
  pub fn create_txt_content(
    content: String,
    option: ParseOption,
    filename: Option<String>,
  ) -> Result<FileInfo, String> {
    let text_content: String = content.clone();
    let charlist: Vec<String> = text_content.tocharlist();
    let mut locmap: Option<LocMap> = None;
    if option.sourcemap {
      locmap = Some(FileInfo::get_loc_by_content(content.as_str()));
    }
    let abs_path = match filename {
      None => cmd_path_resolve("_virtual.less"),
      Some(path_val) => path_val,
    };
    let mut obj = FileInfo {
      disk_location: Some(abs_path),
      block_node: vec![],
      origin_txt_content: text_content,
      origin_charlist: charlist,
      locmap,
      option,
      import_file: vec![],
    };
    match obj.parse() {
      Ok(_) => {}
      Err(msg) => {
        return Err(msg);
      }
    }
    Ok(obj)
  }

  fn parse(&mut self) -> Result<(), String> {
    match self.parse_comment() {
      Ok(blocks) => {
        let mut enum_cc = blocks
          .into_iter()
          .map(StyleNode::Comment)
          .collect::<Vec<StyleNode>>();
        self.block_node.append(&mut enum_cc);
      }
      Err(msg) => {
        return Err(msg);
      }
    }
    match self.parse_var() {
      Ok(blocks) => {
        let mut enum_var = blocks
          .into_iter()
          .map(StyleNode::Var)
          .collect::<Vec<StyleNode>>();
        self.block_node.append(&mut enum_var);
      }
      Err(msg) => {
        return Err(msg);
      }
    }

    match self.parse_rule() {
      Ok(blocks) => {
        let mut enum_rule = blocks
          .into_iter()
          .map(StyleNode::Rule)
          .collect::<Vec<StyleNode>>();
        self.block_node.append(&mut enum_rule);
      }
      Err(msg) => {
        return Err(msg);
      }
    }
    Ok(())
  }
}
