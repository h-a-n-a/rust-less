use crate::extend::string::StringExtend;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::{FileInfo, FileRef};
use crate::new_less::loc::LocMap;
use crate::new_less::node::{NodeRef, StyleNode};
use crate::new_less::parse::Parse;
use crate::new_less::select_node::SelectorNode;
use serde::Serialize;
use std::collections::HashMap;
use std::rc::Rc;
use serde_json::{Map, Value};

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
        let has_codegen_record = {
          let context = info.context.lock().unwrap();
          context.has_codegen_record(&item.info.borrow().disk_location)
        };
        if !has_codegen_record
        {
          let import_res = item.code_gen()?;
          res += &import_res;
          res += "\n";
        }
      }
    }
    let mut self_code_gen_res = "".to_string();

    let source = {
      info.context.lock().unwrap()
        .get_render_cache(info.disk_location.as_str())
    };
    if let Some(code) = source
    {
      self_code_gen_res = code.to_string();
    } else {
      for item in self.getrules() {
        item.borrow().code_gen(&mut self_code_gen_res)?;
      }
      need_add_cache = true;
    }
    res += self_code_gen_res.as_str();
    let mut context = info.context.lock().unwrap();
    context.add_codegen_record(info.disk_location.as_str());
    // 增加缓存
    if need_add_cache {
      context.add_render_cache(info.disk_location.as_str(), self_code_gen_res.as_str());
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
        let has_codegen_record = {
          let context = info.context.lock().unwrap();
          context.has_codegen_record(&item.info.borrow().disk_location)
        };
        if !has_codegen_record
        {
          item.code_gen_into_map(map)?;
        }
      }
    }
    let mut res = "".to_string();
    let source = {
      info.context.lock().unwrap()
        .get_render_cache(info.disk_location.as_str())
    };
    if let Some(code) = source
    {
      res = code.to_string();
    } else {
      for item in self.getrules() {
        item.borrow().code_gen(&mut res)?;
      }
      need_add_cache = true;
    }
    // 增加缓存
    let mut context = info.context.lock().unwrap();
    if need_add_cache {
      context.add_render_cache(info.disk_location.as_str(), res.as_str());
    }
    map.insert(self.info.borrow().disk_location.clone(), res);
    context.add_codegen_record(info.disk_location.as_str());
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
    let cp_context = context.clone();
    let context_value = cp_context.lock().unwrap();
    let option = context_value.get_options();
    drop(context_value);
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
    let mut obj = Self { info: info.clone() };
    obj.parse_heap()?;
    obj.parse_select_all_node()?;
    // 把当前 节点 的 对象 指针 放到 节点上 缓存中
    let disk_location = info.borrow().disk_location.clone();
    let file_info_json = serde_json::to_string_pretty(&obj).unwrap();
    let mut context_value = cp_context.lock().unwrap();
    context_value.set_parse_cache(disk_location.as_str(), file_info_json);
    Ok(obj)
  }


  ///
  /// 递归调用 json 反序列化 自制方法
  ///
  pub fn deserializer(map: &Map<String, Value>, context: ParseContext) -> Result<Self, String> {
    let json_disk_location = map.get("disk_location");
    let json_origin_txt_content = map.get("origin_txt_content");
    let mut obj = FileInfo {
      disk_location: "".to_string(),
      block_node: vec![],
      origin_txt_content: "".to_string(),
      origin_charlist: vec![],
      locmap: None,
      context: context.clone(),
      self_weak: None,
      import_files: vec![],
    };
    if let Some(Value::String(disk_location)) = json_disk_location {
      obj.disk_location = disk_location.to_string();
    } else {
      return Err(format!("deserializer FileNode -> disk_location is empty!"));
    }
    if let Some(Value::String(origin_txt_content)) = json_origin_txt_content {
      obj.origin_txt_content = origin_txt_content.to_string();
      obj.origin_charlist = obj.origin_txt_content.tocharlist();
    } else {
      return Err(format!("deserializer FileNode -> origin_txt_content is empty!"));
    }
    if context.lock().unwrap().option.sourcemap {
      obj.locmap = Some(FileInfo::get_loc_by_content(&obj.origin_charlist));
    }
    let json_import_files = map.get("import_file");
    if let Some(Value::Array(disk_location)) = json_import_files {
      for json_item in disk_location {
        if let Value::Object(json_import_file_node) = json_item {
          let import_info = json_import_file_node.get("info").unwrap().as_object().unwrap();
          obj.import_files.push(Self::deserializer(import_info, context.clone())?);
        }
      }
    }
    let info = obj.toheap();
    let json_block_node = map.get("block_node");
    let mut block_node_recovery_list = vec![];
    if let Some(Value::Array(block_nodes)) = json_block_node {
      for json_node in block_nodes {
        if let Value::Object(json_stylenode) = json_node {
          block_node_recovery_list.push(StyleNode::deserializer(json_stylenode, context.clone(), None, Some(Rc::downgrade(&info)))?);
        }
      }
    }
    info.borrow_mut().block_node = block_node_recovery_list;
    let node = Self { info };
    Ok(node)
  }

  ///
  /// 根据文件路径 转换 文件
  ///
  pub fn create_disklocation(filepath: String, context: ParseContext) -> Result<String, String> {
    let obj = Self::create_disklocation_parse(filepath, context.clone())?;
    let res = obj.code_gen()?;
    let mut sync_context = context.lock().unwrap();
    sync_context.clear_parse_cache();
    sync_context.clear_codegen_record();
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
    match obj.code_gen_into_map(&mut map) {
      Ok(_) => {}
      Err(msg) => {
        println!("{}", msg);
      }
    };
    let mut sync_context = context.lock().unwrap();
    sync_context.clear_parse_cache();
    sync_context.clear_codegen_record();
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
    let cp_context = context.clone();
    let mut sync_context = cp_context.lock().unwrap();
    let option = sync_context.get_options();
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
    let mut obj = Self { info: info.clone() };
    obj.parse_heap()?;
    obj.parse_select_all_node()?;
    // 把当前 节点 的 对象 指针 放到 节点上 缓存中
    let disk_location = info.borrow().disk_location.clone();
    let file_info_json = serde_json::to_string_pretty(&obj).unwrap();
    sync_context.set_parse_cache(disk_location.as_str(), file_info_json);
    Ok(obj)
  }

  pub fn create_txt_content(
    content: String,
    context: ParseContext,
    filename: String,
  ) -> Result<String, String> {
    let obj = Self::create_txt_content_parse(content, context.clone(), filename)?;
    let mut sync_context = context.lock().unwrap();
    let res = obj.code_gen()?;
    sync_context.clear_codegen_record();
    Ok(res)
  }
}
