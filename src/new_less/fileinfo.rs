use crate::extend::string::StringExtend;
use crate::new_less::comment::Comment;
use crate::new_less::context::ParseContext;
use crate::new_less::file_manger::FileManger;
use crate::new_less::loc::LocMap;
use crate::new_less::node::{NodeRef, StyleNode, StyleNodeJson};
use crate::new_less::rule::Rule;
use crate::new_less::var::Var;
use derivative::Derivative;
use serde::Serialize;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct FileInfo {
  // 文件的磁盘位置
  pub disk_location: String,
  // 文件的原始内容
  pub origin_txt_content: String,
  // 根据 原始内容 -> 转化的 字符数组
  pub origin_charlist: Vec<String>,
  // 文件的 原始AST节点
  pub block_node: Vec<StyleNode>,
  // 当前所有 索引 对应的 坐标行列 -> 用于执行 sourcemap
  pub locmap: Option<LocMap>,
  // 全局上下文
  #[derivative(Debug = "ignore")]
  pub context: ParseContext,
  // 自身弱引用
  #[derivative(Debug = "ignore")]
  pub self_weak: FileWeakRef,
  // 该文件的引用文件
  pub import_files: Vec<FileRef>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileInfoJson {
  pub disk_location: String,
  pub block_node: Vec<StyleNodeJson>,
  pub import_file: Vec<FileInfoJson>,
}

pub type FileRef = Rc<RefCell<FileInfo>>;

pub type FileWeakRef = Option<Weak<RefCell<FileInfo>>>;

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
      .import_files
      .iter()
      .map(|x| x.deref().borrow().tojson())
      .collect();
    FileInfoJson {
      disk_location: self.disk_location.clone(),
      block_node,
      import_file,
    }
  }

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
  pub fn get_loc_by_content(content: &str) -> LocMap {
    LocMap::new(content.to_string())
  }

  ///
  /// 根据文件路径 转换 文件
  ///
  pub fn create_disklocation(filepath: String, context: ParseContext) -> Result<String, String> {
    let obj_heap = Self::create_disklocation_parse(filepath, context)?;
    obj_heap
      .deref()
      .borrow()
      .context
      .borrow_mut()
      .clear_codegen();
    let res = match obj_heap.deref().borrow().code_gen() {
      Ok(res) => Ok(res),
      Err(msg) => Err(msg),
    };
    obj_heap
      .deref()
      .borrow()
      .context
      .borrow_mut()
      .clear_codegen();
    res
  }

  ///
  /// 根据文件路径 解析 文件
  ///
  pub fn create_disklocation_parse(
    filepath: String,
    context: ParseContext,
  ) -> Result<FileRef, String> {
    let text_content: String;
    let charlist: Vec<String>;
    let mut locmap: Option<LocMap> = None;
    let option = context.deref().borrow().get_options();
    let obj = match FileManger::resolve(filepath, option.include_path.clone()) {
      Ok((abs_path, content)) => {
        text_content = content.clone();
        if option.sourcemap {
          locmap = Some(FileInfo::get_loc_by_content(content.as_str()));
        }
        charlist = content.tocharlist();
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
    Self::parse_heap(obj_heap.clone())?;
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
    let text_content: String = content.clone();
    let charlist: Vec<String> = text_content.tocharlist();
    let option = context.deref().borrow().get_options();
    let mut locmap: Option<LocMap> = None;
    if option.sourcemap {
      locmap = Some(FileInfo::get_loc_by_content(content.as_str()));
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
    match Self::parse_heap(obj_heap.clone()) {
      Ok(_) => {}
      Err(msg) => {
        return Err(msg);
      }
    }
    Ok(obj_heap)
  }

  pub fn create_txt_content(
    content: String,
    context: ParseContext,
    filename: String,
  ) -> Result<String, String> {
    let obj = Self::create_txt_content_parse(content, context, filename)?;
    let res = match obj.deref().borrow().code_gen() {
      Ok(res) => Ok(res),
      Err(msg) => Err(msg),
    };
    res
  }

  ///
  /// 转化 AST
  ///
  pub fn parse_heap(obj: FileRef) -> Result<(), String> {
    // 把当前 节点 的 对象 指针 放到 节点上 缓存中
    let disk_location_path = obj.deref().borrow().disk_location.clone();
    obj.deref().borrow().context.borrow_mut().set_cache(
      disk_location_path.as_str(),
      obj.deref().borrow().self_weak.clone(),
    );
    // 开始转换
    obj.deref().borrow_mut().parse_comment()?;
    obj.deref().borrow_mut().parse_var()?;
    obj.deref().borrow_mut().parse_rule()?;
    Ok(())
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
      item.deref().borrow().code_gen(&mut res);
    }
    Ok(res)
  }
}
