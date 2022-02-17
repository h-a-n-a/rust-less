use crate::new_less::file_manger::FileManger;
use crate::new_less::fileinfo::{FileInfo, FileRef, FileWeakRef};
use crate::new_less::option::ParseOption;
use derivative::Derivative;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

pub type ParseCacheMap = HashMap<String, FileWeakRef>;

pub type ParseContext = Rc<RefCell<Context>>;

///
/// 全局调用 转化时的 上下文
///
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Context {
  // 内部调用方式时 需要拿到对应的 转化配置
  pub option: ParseOption,
  // 转文件 的缓存
  #[derivative(Debug = "ignore")]
  pub filecache: ParseCacheMap,
  // 文件的绝对路径 入口文件
  pub application_fold: String,
  // 已经生成目录的 文件
  pub code_gen_file_path: Vec<String>,
}

impl Context {
  ///
  /// 创建全局应用 共享上下文
  ///
  pub fn new(option: ParseOption, application_fold: Option<String>) -> Result<Self, String> {
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
          let current_dir = FileManger::get_dir(&fold)?;
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
      filecache: HashMap::new(),
      application_fold: fold.clone(),
      code_gen_file_path: vec![],
    };
    obj.set_include_paths(vec![fold]);
    Ok(obj)
  }

  ///
  /// 查询 缓存上 翻译结果
  ///
  pub fn get_cache(&self, file_path: &str) -> FileWeakRef {
    let map = &self.filecache;
    let res = map.get(file_path);
    res.map(|x| x.clone().as_ref().unwrap().clone())
  }

  ///
  /// 添加 缓存上 翻译结果
  ///
  pub fn set_cache(&mut self, file_path: &str, file_weak_ref: FileWeakRef) {
    let res = self.filecache.get(file_path);
    if res.is_none() {
      self.filecache.insert(file_path.to_string(), file_weak_ref);
    }
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
  /// 产生代码
  ///
  pub fn render(self, filepath: String) -> Result<String, String> {
    let context = Rc::new(RefCell::new(self));
    FileInfo::create_disklocation(filepath, context)
  }

  ///
  /// 析构代码
  ///
  pub fn parse(self, filepath: String) -> Result<FileRef, String> {
    let context = Rc::new(RefCell::new(self));
    FileInfo::create_disklocation_parse(filepath, context)
  }

  ///
  /// 生成默认上下文
  ///
  pub fn default() -> ParseContext {
    let obj = Self::new(Default::default(), None).unwrap();
    Rc::new(RefCell::new(obj))
  }

  ///
  /// 清楚生成 文件名
  ///
  pub fn clear_codegen(&mut self) {
    self.code_gen_file_path.clear();
  }

  ///
  /// 是否已经生成了
  ///
  pub fn has_codegen(&self, path: &str) -> bool {
    self.code_gen_file_path.contains(&path.to_string())
  }
}
