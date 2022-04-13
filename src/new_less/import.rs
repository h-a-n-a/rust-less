use crate::extend::vec_str::VecCharExtend;
use crate::new_less::context::ParseContext;
use crate::new_less::file_manger::FileManger;
use crate::new_less::fileinfo::{FileInfo, FileRef, FileWeakRef};
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::NodeWeakRef;
use crate::new_less::option::ParseOption;
use crate::new_less::scan::traversal;
use crate::new_less::token::lib::Token;
use crate::new_less::var::HandleResult;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

///
/// import 处理
///
#[derive(Clone)]
pub struct ImportNode {
  // 节点坐标
  pub loc: Option<Loc>,

  // 内部处理 地图
  map: LocMap,

  // 自身 Rule 的弱引用
  parent: NodeWeakRef,

  // 文件信息
  pub fileinfo: FileWeakRef,

  // 内部快速扫词 字符串 数组
  charlist: Vec<char>,

  // 经常 插件 hook 的 计算完的 文件地址
  parse_hook_url: String,

  // 上下文
  pub context: ParseContext,
}

impl Debug for ImportNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ImportNode")
      .field("content", &self.charlist.poly())
      .field("loc", &self.loc)
      .field("path", &self.parse_hook_url)
      .finish()
  }
}

impl Serialize for ImportNode {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("ImportNode", 3)?;
    state.serialize_field("content", &self.charlist.poly())?;
    state.serialize_field("loc", &self.loc)?;
    state.serialize_field("path", &self.parse_hook_url)?;
    state.end()
  }
}

impl ImportNode {
  ///
  /// 初始化方法
  ///
  pub fn new(
    charlist: Vec<char>,
    loc: Option<Loc>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
    context: ParseContext,
    importfiles: &mut Vec<FileRef>,
  ) -> HandleResult<Self> {
    let map = if loc.is_none() {
      LocMap::new(&charlist)
    } else {
      LocMap::merge(loc.as_ref().unwrap(), &charlist).0
    };
    let mut obj = Self {
      loc,
      map,
      parent,
      fileinfo,
      charlist,
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
      &self.charlist.poly(),
      char,
      error_loc.line,
      error_loc.col
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
        let (index, temp, hasend) = arg;

        if has_apost || has_quote {
          if Token::is_token(Some(char)) {
            if ('\'' == *char && has_apost) || ('"' == *char && has_quote) {
              if *index != charlist.len() - 2 {
                return Err(self.error_msg(index));
              } else {
                has_apost = false;
                has_quote = false;
                *hasend = true
              }
            } else {
              temp.push(*char);
            }
          } else {
            temp.push(*char);
          }
        } else if Token::is_token(Some(char)) {
          if !Token::is_space_token(Some(char)) {
            if '\'' == *char {
              has_apost = true;
            } else if '"' == *char {
              has_quote = true;
            } else {
              return Err(self.error_msg(index));
            }
          }
        } else {
          return Err(self.error_msg(index));
        }
        Ok(())
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
