use crate::extend::string::StringExtend;
use crate::new_less::context::ParseContext;
use crate::new_less::file_manger::FileManger;
use crate::new_less::fileinfo::{FileInfo, FileRef, FileWeakRef};
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{HandleResult, NodeWeakRef};
use crate::new_less::option::ParseOption;
use crate::new_less::scan::{traversal, ScanArg, ScanResult};
use crate::new_less::token::lib::Token;
use derivative::Derivative;
use serde::Serialize;
use std::rc::Rc;

///
/// import 处理
///
#[derive(Derivative, Serialize, Clone)]
#[derivative(Debug)]
pub struct ImportNode {
  // 原始字符
  #[serde(rename(serialize = "content"))]
  pub origin_txt: String,

  // 节点坐标
  pub loc: Option<Loc>,

  // 内部处理 地图
  #[serde(skip_serializing)]
  map: LocMap,

  // 自身 Rule 的弱引用
  #[serde(skip_serializing)]
  parent: NodeWeakRef,

  // 文件信息
  #[serde(skip_serializing)]
  pub fileinfo: FileWeakRef,

  // 内部快速扫词 字符串 数组
  #[serde(skip_serializing)]
  charlist: Vec<char>,

  // 经常 插件 hook 的 计算完的 文件地址
  #[serde(rename(serialize = "path"))]
  parse_hook_url: String,

  // 上下文
  #[derivative(Debug = "ignore")]
  #[serde(skip_serializing)]
  pub context: ParseContext,
}

impl ImportNode {
  ///
  /// 初始化方法
  ///
  pub fn new(
    txt: String,
    loc: Option<Loc>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
    context: ParseContext,
    importfiles: &mut Vec<FileRef>,
  ) -> HandleResult<Self> {
    let map = if loc.is_none() {
      LocMap::new(txt.clone())
    } else {
      LocMap::merge(loc.as_ref().unwrap(), &txt).0
    };
    let mut obj = Self {
      origin_txt: txt.to_string(),
      loc,
      map,
      parent,
      fileinfo,
      charlist: txt.trim().to_string().tocharlist(),
      parse_hook_url: "".to_string(),
      context,
    };
    match obj.parse(importfiles) {
      Ok(_) => HandleResult::Success(obj),
      Err(msg) => HandleResult::Fail(msg),
    }
  }

  ///
  /// 判断是否是 顶层 节点 下的变量
  ///
  pub fn is_top(&self) -> bool {
    self.parent.is_none()
  }

  ///
  /// 报错信息
  ///
  pub fn error_msg(&self, index: &usize) -> String {
    let error_loc = self.map.get(index).unwrap();
    let char = self.charlist.get(*index).unwrap().to_string();
    format!(
      "text {}, char {} is not allow, line is {} col is {}",
      &self.origin_txt, char, error_loc.line, error_loc.col
    )
  }

  ///
  /// 解析 字符串
  ///
  fn parse(&mut self, importfiles: &mut Vec<FileRef>) -> Result<(), String> {
    let charlist = &self.charlist.clone();
    let index = 7;
    let mut has_apost = false;
    let mut has_quote = false;

    // 遍历 扫词
    let path = match traversal(
      Some(index),
      charlist,
      &mut (|arg, (_, char, _)| {
        let ScanArg {
          index,
          mut temp,
          mut hasend,
        } = arg;

        if has_apost || has_quote {
          if Token::is_token(&char) {
            if ('\'' == *char && has_apost)
              || ('"' == *char && has_quote)
            {
              if index != charlist.len() - 2 {
                return Err(self.error_msg(&index));
              } else {
                has_apost = false;
                has_quote = false;
                hasend = true
              }
            } else {
              temp.push(char.clone());
            }
          } else {
            temp.push(char.clone());
          }
        } else if Token::is_token(&char) {
          if !Token::is_space_token(&char) {
            if '\'' == *char {
              has_apost = true;
            } else if '"' == *char {
              has_quote = true;
            } else {
              return Err(self.error_msg(&index));
            }
          }
        } else {
          return Err(self.error_msg(&index));
        }

        Ok(ScanResult::Arg(ScanArg {
          index,
          temp,
          hasend,
        }))
      }),
    ) {
      Ok(res) => res.0,
      Err(msg) => {
        return Err(msg);
      }
    };
    if has_apost || has_quote {
      return Err(self.error_msg(&(self.charlist.len() - 2)));
    }
    let options = self.get_options();
    // 过滤 对应 hook
    if options.hooks.import_alias.is_some() {
      let convert = options.hooks.import_alias.as_ref().unwrap();
      self.parse_hook_url = convert(path);
    } else {
      self.parse_hook_url = path;
    }
    // 处理递归解析 若节点不存在 则 不进行处理
    let file_path = self.parse_hook_url.clone();
    let include_path = self.get_options().include_path;
    let (abs_path, _file_content) = FileManger::resolve(file_path, include_path)?;
    let weak_file_ref_option = self.context.borrow().get_cache(abs_path.as_str());
    // 自动忽略已经翻译后的文件
    if let Some(weak_file_ref) = weak_file_ref_option {
      let heap_obj = weak_file_ref.upgrade().unwrap();
      importfiles.push(heap_obj);
    } else {
      let heap_obj = FileInfo::create_disklocation_parse(abs_path.clone(), self.context.clone())?;
      importfiles.push(heap_obj.clone());
      self
        .context
        .borrow_mut()
        .set_cache(abs_path.as_str(), Some(Rc::downgrade(&heap_obj)));
    }
    Ok(())
  }

  ///
  /// 获取选项
  ///
  pub fn get_options(&self) -> ParseOption {
    self.context.borrow().option.clone()
  }
}
