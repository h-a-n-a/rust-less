use crate::extend::vec_str::VecCharExtend;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::NodeWeakRef;
use crate::new_less::scan::{ScanArg, ScanResult, traversal};
use crate::new_less::var::HandleResult;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct NewSelector {
  // 字符串规则 根据逗号分割
  pub single_select_txt: Vec<String>,

  // 坐标位置
  pub loc: Option<Loc>,

  // 内部处理 地图
  #[serde(skip_serializing)]
  map: LocMap,

  // 字符串 操作 序列
  #[serde(skip_serializing)]
  pub charlist: Vec<char>,

  // 节点 父节点
  // 延迟赋值
  #[serde(skip_serializing)]
  pub parent: NodeWeakRef,
}


impl NewSelector {

  pub fn new(
    charlist: Vec<char>,
    loc: Option<Loc>,
    map: Option<LocMap>,
    parent: NodeWeakRef,
  ) -> HandleResult<Self> {
    let mut obj = NewSelector {
      single_select_txt: vec![],
      loc,
      map: map.unwrap_or_else(|| LocMap::new(&charlist)),
      charlist,
      parent,
    };
    match obj.parse() {
      Ok(()) => HandleResult::Success(obj),
      Err(msg) => HandleResult::Fail(msg),
    }
  }

  pub fn value(&self) -> String {
    self.charlist.poly()
  }

  ///
  /// 打印错误信息
  ///
  fn errormsg(&mut self, index: &usize) -> Result<(), String> {
    let char = *self.charlist.get(*index).unwrap();
    let error_loc = self.map.get(index).unwrap();
    Err(format!(
      "select text {}, char {} is not allow, line is {} col is {}",
      self.charlist.poly(),
      char,
      error_loc.line,
      error_loc.col
    ))
  }

  fn parse(&mut self) -> Result<(), String> {
    let charlist = &self.charlist;
    let index: usize = 0;
    let res = traversal(
      Some(index),
      charlist,
      &mut (|arg, charword| {
        let ScanArg {
          temp,
          mut index,
          mut hasend,
        } = arg;
        Ok(ScanResult::Arg(ScanArg {
          index,
          temp,
          hasend,
        }))
      }))?;
    Ok(())
  }

}