use crate::extend::string::StringExtend;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::{FileInfo, FileRef};
use crate::new_less::loc::LocMap;
use crate::new_less::node::{NodeRef, StyleNode};
use crate::new_less::parse::Parse;
use crate::new_less::select_node::SelectorNode;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct FileNode {
  pub info: FileRef,
}

impl FileNode {
  ///
  /// 收集当前节点下所有的Rule
  ///
  pub fn getrules(&self) -> Vec<NodeRef> {
    let mut list = vec![];
    self.info.borrow().block_node.iter().for_each(|x| {
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
    let info = self.info.borrow();
    if !info.import_files.is_empty() {
      for item in info.import_files.iter() {
        if !info
          .context
          .borrow()
          .has_codegen(&item.info.borrow().disk_location)
        {
          let import_res = item.code_gen()?;
          res += &import_res;
          res += "\n";
        }
      }
    }
    for item in self.getrules() {
      item.borrow().code_gen(&mut res)?;
    }
    Ok(res)
  }

  ///
  /// parse 当前文件下 所有的 select 字符串
  /// 需要 第一遍 完成基本遍历
  ///
  pub fn parse_select_all_node(&self) -> Result<(), String> {
    // todo! 若要支持 @{abc} 变量 跨文件调用 select 需要 select 解析放到 codegen 里
    for node in self.info.borrow().block_node.iter() {
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

  ///
  /// 根据文件路径 解析 文件
  ///
  pub fn create_disklocation_parse(
    filepath: String,
    context: ParseContext,
  ) -> Result<FileNode, String> {
    let text_content: String;
    let charlist: Vec<char>;
    let mut locmap: Option<LocMap> = None;
    let option = context.borrow().get_options();
    let obj = match FileInfo::resolve(filepath, &option.include_path) {
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
    let info = obj.toheap();
    let mut obj = Self { info };
    obj.parse_heap()?;
    obj.parse_select_all_node()?;
    Ok(obj)
  }

  ///
  /// 根据文件路径 转换 文件
  ///
  pub fn create_disklocation(filepath: String, context: ParseContext) -> Result<String, String> {
    let obj = Self::create_disklocation_parse(filepath, context)?;
    let res = obj.code_gen()?;
    Ok(res)
  }

  ///
  /// 根据文件内容 解析文件
  ///
  pub fn create_txt_content_parse(
    content: String,
    context: ParseContext,
    filename: String,
  ) -> Result<Self, String> {
    let text_content: String = content;
    let charlist = text_content.tocharlist();
    let option = context.borrow().get_options();
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
    let info = obj.toheap();
    let mut obj = Self { info };
    obj.parse_heap()?;
    obj.parse_select_all_node()?;
    Ok(obj)
  }

  pub fn create_txt_content(
    content: String,
    context: ParseContext,
    filename: String,
  ) -> Result<String, String> {
    let obj = Self::create_txt_content_parse(content, context, filename)?;
    let res = obj.code_gen()?;
    Ok(res)
  }
}
