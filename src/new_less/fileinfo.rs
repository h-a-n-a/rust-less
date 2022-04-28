use crate::extend::string::StringExtend;
use crate::new_less::context::ParseContext;
use crate::new_less::file::{path_join, readfile};
use crate::new_less::file_manger::FileManger;
use crate::new_less::loc::LocMap;
use crate::new_less::node::{NodeRef, StyleNode};
use crate::new_less::parse::Parse;
use crate::new_less::select_node::SelectorNode;
use crate::new_less::var::VarRuleNode;
use crate::new_less::var_node::VarNode;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::path::Path;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct FileInfo {
  // 文件的磁盘位置
  pub disk_location: String,
  // 文件的原始内容
  pub origin_txt_content: String,
  // 根据 原始内容 -> 转化的 字符数组
  pub origin_charlist: Vec<char>,
  // 文件的 原始AST节点
  pub block_node: Vec<StyleNode>,
  // 当前所有 索引 对应的 坐标行列 -> 用于执行 sourcemap
  pub locmap: Option<LocMap>,
  // 全局上下文
  pub context: ParseContext,
  // 自身弱引用
  pub self_weak: FileWeakRef,
  // 该文件的引用文件
  pub import_files: Vec<FileRef>,
}

pub type FileRef = Rc<RefCell<FileInfo>>;

pub type FileWeakRef = Option<Weak<RefCell<FileInfo>>>;

impl Serialize for FileInfo {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("FileInfo", 3)?;
    state.serialize_field("disk_location", &self.disk_location)?;
    state.serialize_field("block_node", &self.block_node)?;
    state.serialize_field("import_file", &self.import_files)?;
    state.end()
  }
}

impl Debug for FileInfo {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("FileInfo")
      .field("disk_location", &self.disk_location)
      .field("block_node", &self.block_node)
      .field("import_file", &self.import_files)
      .finish()
  }
}

impl FileInfo {
  ///
  /// 转 heap 堆上对象
  ///
  pub fn toheap(self) -> FileRef {
    let heapobj = Rc::new(RefCell::new(self));
    heapobj.borrow_mut().self_weak = Some(Rc::downgrade(&heapobj));
    heapobj
  }

  ///
  /// 生成整个文件的 locmap 地图
  ///
  pub fn get_loc_by_content(chars: &[char]) -> LocMap {
    LocMap::new(chars)
  }

  ///
  /// 根据文件路径 转换 文件
  ///
  pub fn create_disklocation(filepath: String, context: ParseContext) -> Result<String, String> {
    let obj_heap = Self::create_disklocation_parse(filepath, context)?;
    let res = obj_heap.deref().borrow().code_gen()?;
    Ok(res)
  }

  ///
  /// 根据文件路径 解析 文件
  ///
  pub fn create_disklocation_parse(
    filepath: String,
    context: ParseContext,
  ) -> Result<FileRef, String> {
    let text_content: String;
    let charlist: Vec<char>;
    let mut locmap: Option<LocMap> = None;
    let option = context.deref().borrow().get_options();
    let obj = match FileManger::resolve(filepath, option.include_path.clone()) {
      Ok((abs_path, content)) => {
        text_content = content.clone();
        charlist = content.tocharlist();
        if option.sourcemap {
          locmap = Some(FileInfo::get_loc_by_content(&charlist));
        }
        FileInfo {
          disk_location: abs_path,
          block_node: vec![],
          origin_txt_content: text_content,
          origin_charlist: charlist,
          locmap,
          context,
          self_weak: None,
          import_files: vec![],
        }
      }
      Err(msg) => {
        return Err(msg);
      }
    };
    let obj_heap = obj.toheap();
    obj_heap.borrow_mut().parse_heap()?;
    obj_heap.borrow().parse_select_all_node()?;
    Ok(obj_heap)
  }

  ///
  /// 根据文件内容 解析文件
  ///
  pub fn create_txt_content_parse(
    content: String,
    context: ParseContext,
    filename: String,
  ) -> Result<FileRef, String> {
    let text_content: String = content;
    let charlist = text_content.tocharlist();
    let option = context.deref().borrow().get_options();
    let mut locmap: Option<LocMap> = None;
    if option.sourcemap {
      locmap = Some(FileInfo::get_loc_by_content(&charlist));
    }
    let obj = FileInfo {
      disk_location: filename,
      block_node: vec![],
      origin_txt_content: text_content,
      origin_charlist: charlist,
      locmap,
      context,
      self_weak: None,
      import_files: vec![],
    };
    let obj_heap = obj.toheap();
    obj_heap.borrow_mut().parse_heap()?;
    obj_heap.borrow().parse_select_all_node()?;
    Ok(obj_heap)
  }

  ///
  /// parse 当前文件下 所有的 select 字符串
  /// 需要 第一遍 完成基本遍历
  ///
  pub fn parse_select_all_node(&self) -> Result<(), String> {
    // todo! 若要支持 @{abc} 变量 跨文件调用 select 需要 select 解析放到 codegen 里
    for node in self.block_node.iter() {
      if let StyleNode::Rule(heapnode) = node {
        let mut mut_node = heapnode.borrow_mut();
        if let Some(SelectorNode::Select(s_node)) = mut_node.selector.as_mut() {
          s_node.parse(None)?;
        }
        drop(mut_node);
        heapnode.borrow().parse_select_all_node()?;
      }
    }
    Ok(())
  }

  pub fn create_txt_content(
    content: String,
    context: ParseContext,
    filename: String,
  ) -> Result<String, String> {
    let obj = Self::create_txt_content_parse(content, context, filename)?;
    let res = obj.deref().borrow().code_gen()?;
    Ok(res)
  }

  pub fn getrules(&self) -> Vec<NodeRef> {
    let mut list = vec![];

    self.block_node.iter().for_each(|x| {
      if let StyleNode::Rule(rule) = x {
        list.push(rule.clone())
      }
    });
    list
  }

  ///
  /// 生成代码
  ///
  pub fn code_gen(&self) -> Result<String, String> {
    let mut res = "".to_string();
    if !self.import_files.is_empty() {
      for item in self.import_files.iter() {
        if !self
          .context
          .borrow()
          .has_codegen(&item.deref().borrow().disk_location)
        {
          let import_res = item.deref().borrow().code_gen()?;
          res += &import_res;
          res += "\n";
        }
      }
    }
    for item in self.getrules() {
      item.deref().borrow().code_gen(&mut res)?;
    }
    Ok(res)
  }

  ///
  /// 获取 某文件下 所有的 变量节点
  /// 递归 获取所有 fileinfo 上 block_node -> var 节点
  ///
  pub fn collect_vars(&self) -> Vec<VarNode> {
    let mut varlist = vec![];
    for fileinfo in &self.import_files {
      for item in &fileinfo.borrow().block_node {
        if let StyleNode::Var(VarRuleNode::Var(var)) = item.deref() {
          varlist.push(var.clone());
        }
      }
      // 递归收集
      let mut child_var_list = fileinfo.borrow().collect_vars();
      varlist.append(&mut child_var_list)
    }
    varlist
  }

  ///
  /// 修复路径
  ///
  pub fn save_include_paths_with_context(&mut self, filepath: &str) {
    if !Self::is_relative_path(filepath) {
      if self
        .context
        .borrow()
        .option
        .include_path
        .contains(filepath.as_ref())
      {
        return;
      } else {
        self
          .context
          .borrow_mut()
          .option
          .include_path
          .push(filepath.to_string());
      }
    } else {
    }
  }

  ///
  /// 获取指定文件的路径
  /// 如果是路径 -> 直接返回该路径
  ///
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

  ///
  /// 是否是相对路径
  ///
  pub fn is_relative_path(txt: &str) -> bool {
    let path = Path::new(txt);
    path.is_relative()
  }

  ///
  /// 文件查找对应解析路径
  /// 返回值 -> (路径, 文件内容)
  ///
  pub fn resolve(&self, filepath: String) -> Result<(String, String), String> {
    // 相对路径 和 绝对路径 分开计算
    return if FileManger::is_relative_path(&filepath) {
      // 相对路径的情况
      let mut abs_path: Option<String> = None;
      let mut failpath = vec![];
      let mut content: Option<String> = None;
      for basepath in self.context.borrow().option.include_path {
        let temp_path = path_join(basepath.as_str(), filepath.as_str());
        match readfile(temp_path.as_str()) {
          Ok(res) => {
            content = Some(res);
            abs_path = Some(temp_path.clone());
            break;
          }
          Err(_) => failpath.push(temp_path.clone()),
        }
      }
      return if let Some(match_path) = abs_path {
        Ok((match_path, content.unwrap()))
      } else {
        Err(format!(
          "Nothings File is find in cmdpath and inculdepath,{}",
          failpath.join(";")
        ))
      };
    } else {
      // 绝对路径的情况
      let res = readfile(filepath.as_str())?;
      Ok((filepath.clone(), res))
    };
  }
}
