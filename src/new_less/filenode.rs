use crate::extend::string::StringExtend;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::{FileInfo, FileRef};
use crate::new_less::loc::LocMap;
use crate::new_less::node::{NodeRef, StyleNode};
use crate::new_less::parse::Parse;
use crate::new_less::select_node::SelectorNode;
use serde::Serialize;
use std::collections::HashMap;

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
    let mut need_add_cache = false;
    if !info.import_files.is_empty() {
      for item in info.import_files.iter() {
        if !info
          .context
          .borrow()
          .has_codegen_record(&item.info.borrow().disk_location)
        {
          let import_res = item.code_gen()?;
          res += &import_res;
          res += "\n";
        }
      }
    }
    let mut self_code_gen_res = "".to_string();
    if let Some(source) = info
      .context
      .borrow()
      .get_render_cache(info.disk_location.as_str())
    {
      self_code_gen_res = source.to_string();
    } else {
      for item in self.getrules() {
        item.borrow().code_gen(&mut self_code_gen_res)?;
      }
      need_add_cache = true;
    }
    res += self_code_gen_res.as_str();
    info
      .context
      .borrow_mut()
      .add_codegen_record(info.disk_location.as_str());
    // 增加缓存
    if need_add_cache {
      info
        .context
        .borrow_mut()
        .add_render_cache(info.disk_location.as_str(), self_code_gen_res.as_str());
    }
    Ok(res)
  }

  ///
  /// 用来执行多 css 之间的 bundle
  /// 初始化 执行 需要 放入一个 空map 的引用 后续会自动填充到 该参数 上
  ///
  pub fn code_gen_into_map(&self, map: &mut HashMap<String, String>) -> Result<(), String> {
    let info = self.info.borrow();
    let mut need_add_cache = false;
    if !info.import_files.is_empty() {
      for item in info.import_files.iter() {
        if !info
          .context
          .borrow()
          .has_codegen_record(&item.info.borrow().disk_location)
        {
          item.code_gen_into_map(map)?;
        }
      }
    }
    let mut res = "".to_string();
    if let Some(source) = info
      .context
      .borrow()
      .get_render_cache(info.disk_location.as_str())
    {
      res = source.to_string();
    } else {
      for item in self.getrules() {
        item.borrow().code_gen(&mut res)?;
      }
      need_add_cache = true;
    }
    // 增加缓存
    if need_add_cache {
      info
        .context
        .borrow_mut()
        .add_render_cache(info.disk_location.as_str(), res.as_str());
    }
    map.insert(self.info.borrow().disk_location.clone(), res);
    info
      .context
      .borrow_mut()
      .add_codegen_record(info.disk_location.as_str());
    Ok(())
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
    let obj = Self::create_disklocation_parse(filepath, context.clone())?;
    let res = obj.code_gen()?;
    context.borrow_mut().clear_parse_cache();
    context.borrow_mut().clear_codegen_record();
    Ok(res)
  }

  ///
  /// 根据文件路径 转换 文件
  ///
  pub fn create_disklocation_into_hashmap(
    filepath: String,
    context: ParseContext,
  ) -> Result<HashMap<String, String>, String> {
    let obj = Self::create_disklocation_parse(filepath, context.clone())?;
    let mut map = HashMap::new();
    obj.code_gen_into_map(&mut map)?;
    context.borrow_mut().clear_parse_cache();
    context.borrow_mut().clear_codegen_record();
    Ok(map)
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
    let obj = Self::create_txt_content_parse(content, context.clone(), filename)?;
    let res = obj.code_gen()?;
    context.borrow_mut().clear_codegen_record();
    Ok(res)
  }
}
