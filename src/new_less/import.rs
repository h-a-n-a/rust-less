use crate::extend::string::StringExtend;
use crate::new_less::loc::Loc;
use crate::new_less::node::HandleResult;
use serde::Serialize;

///
/// import 处理
///
#[derive(Debug, Clone, Serialize)]
pub struct ImportNode {
  // 原始字符
  pub origin_txt: String,

  // 节点坐标
  pub loc: Option<Loc>,

  // 内部快速扫词 字符串 数组
  #[serde(skip_serializing)]
  charlist: Vec<String>,
}

impl ImportNode {
  ///
  /// 初始化方法
  ///
  pub fn new(txt: String, loc: Option<Loc>) -> HandleResult<Self> {
    let _obj = Self {
      origin_txt: txt.to_string(),
      loc,
      charlist: txt.trim().to_string().tocharlist(),
    };
    // match obj.parse() {
    //   Ok(()) => HandleResult::Success(obj),
    //   Err(msg) => HandleResult::Fail(msg),
    // }
    HandleResult::Swtich
  }

  ///
  /// 解析 字符串
  ///
  fn parse(&self) -> Result<(), String> {
    // let charlist = &self.charlist;
    //
    // let length = charlist.len();
    //
    // if length < 7
    //   || (length == 7 && charlist[0..7].poly().as_str() != "@import")
    //   || (length > 7 && charlist[0..8].poly().as_str() != "@import")
    // {
    //   return Err("select_txt not match import".to_string());
    // }
    //
    // let index = 7;
    //
    // while index < charlist.len() {
    //   // let char = charlist.get(index).unwrap().to_string();
    // }
    Ok(())
  }
}
