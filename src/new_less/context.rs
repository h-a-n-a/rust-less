use crate::new_less::fileinfo::{FileInfo, FileRef};
use crate::new_less::option::ParseOption;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::path::Path;
use std::sync::{Arc, Mutex, Weak};

pub type ParseCacheMap = Mutex<HashMap<String, Mutex<FileRef>>>;

pub type ParseContext = Arc<RefCell<Context>>;

pub type RenderCacheMap = Mutex<HashMap<String, String>>;

///
/// 全局调用 转化时的 上下文
///
pub struct Context {
  // 内部调用方式时 需要拿到对应的 转化配置
  pub option: ParseOption,
  // 转文件 的缓存
  pub filecache: ParseCacheMap,
  // 渲染结果 的缓存
  pub render_cache: RenderCacheMap,
  // 文件的绝对路径 入口文件
  pub application_fold: String,
  // 已经生成目录的 文件
  pub code_gen_file_path: Vec<String>,
  // 自身的弱引用
  pub weak_ref: Option<Weak<RefCell<Context>>>,
}

impl Debug for Context {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Context")
      .field("option", &self.option)
      .field("entry", &self.application_fold)
      .field("filepaths", &self.code_gen_file_path)
      .finish()
  }
}

impl Context {
  ///
  /// 创建全局应用 共享上下文
  ///
  pub fn new(option: ParseOption, application_fold: Option<String>) -> Result<Arc<RefCell<Self>>, String> {
    let mut fold = application_fold.unwrap_or_else(|| {
      std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
    });
    let filepath = Path::new(&fold);
    if filepath.exists() {
      if filepath.is_absolute() {
        if filepath.is_file() {
          let current_dir = FileInfo::get_dir(&fold)?;
          fold = current_dir
        } else if !filepath.is_dir() {
          return Err(format!("application_fold is not file or dir,{}", fold));
        }
      } else {
        return Err(format!(
          "application_fold is must absolutely path ,{}",
          fold
        ));
      }
    } else {
      return Err(format!("application_fold is not exists,{}", fold));
    }
    let mut obj = Context {
      option,
      filecache: Mutex::new(HashMap::new()),
      render_cache: Mutex::new(HashMap::new()),
      application_fold: fold.clone(),
      code_gen_file_path: vec![],
      weak_ref: None,
    };
    obj.set_include_paths(vec![fold]);
    let heap_obj = Arc::new(RefCell::new(obj));
    heap_obj.borrow_mut().weak_ref = Some(Arc::downgrade(&heap_obj));
    Ok(heap_obj)
  }

  ///
  /// 查询 缓存上 翻译结果
  ///
  pub fn get_parse_cache(&self, file_path: &str) -> Option<FileRef> {
    let map = &self.filecache;
    let mapvalue = map.try_lock().unwrap();
    let res = mapvalue.get(file_path);
    if let Some(info) = res {
      let p = info.try_lock().unwrap().clone();
      Some(p)
    } else {
      None
    }
  }

  ///
  /// 添加 缓存上 翻译结果
  ///
  pub fn set_parse_cache(&mut self, file_path: &str, file_info: FileRef) {
    let mut filecache = self.filecache.try_lock().unwrap();
    let res = filecache.get(file_path);
    if res.is_none() {
      filecache.insert(file_path.to_string(), Mutex::new(file_info));
    }
  }

  ///
  /// 清除 parse cache
  /// 由于现在 缓存的是 指针 只能 单次 transform 同一个文件多次使用
  ///
  pub fn clear_parse_cache(&mut self) {
    self.filecache.try_lock().unwrap().clear();
  }

  ///
  /// 获取选项
  ///
  pub fn get_options(&self) -> ParseOption {
    self.option.clone()
  }

  ///
  /// 安全设置 include-path
  ///
  pub fn set_include_paths(&mut self, paths: Vec<String>) {
    paths.iter().for_each(|x| {
      if !self.option.include_path.contains(x) {
        self.option.include_path.push(x.clone());
      }
    });
  }

  ///
  /// 增加一个css transform 下 @import 引用生成的记录
  ///
  pub fn add_codegen_record(&mut self, path: &str) {
    self.code_gen_file_path.push(path.to_string());
  }

  ///
  /// 清除本次 codegen 文件的记录
  ///
  pub fn clear_codegen_record(&mut self) {
    self.code_gen_file_path.clear();
  }

  ///
  /// 是否已经生成过 该文件
  ///
  pub fn has_codegen_record(&self, path: &str) -> bool {
    self.code_gen_file_path.contains(&path.to_string())
  }

  ///
  /// 插入 生成 样式文件的缓存
  ///
  pub fn add_render_cache(&mut self, filepath: &str, source: &str) {
    self
      .render_cache.try_lock().unwrap()
      .insert(filepath.to_string(), source.to_string());
  }

  ///
  /// 清除本次 codegen 文件的记录
  ///
  pub fn clear_render_cache(&mut self) {
    self.render_cache.try_lock().unwrap().clear();
  }

  ///
  /// 获取 codegen 缓存 目标样式代码
  ///
  pub fn get_render_cache(&self, filepath: &str) -> Option<String> {
    self.render_cache.try_lock().unwrap().get(filepath).cloned()
  }

  ///
  /// 生成默认上下文
  ///
  pub fn default() -> ParseContext {
    Self::new(Default::default(), None).unwrap()
  }
}
