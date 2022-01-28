use crate::extend::string::StringExtend;
use crate::new_less::loc::Loc;
use crate::new_less::node::HandleResult;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct VarNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Option<Loc>,
}

impl VarNode {
  pub fn new(txt: String, loc: Option<Loc>) -> HandleResult<Self> {
    let obj = Self { content: txt, loc };
    HandleResult::Success::<Self>(obj)
  }
}

// let getmsg = |index: usize| -> String {
// let location_msg = if options.sourcemap {
// format!("loc at {:#?}", locmap.as_ref().unwrap().get(index).unwrap())
// } else {
// format!("word order is {}", index)
// };
// location_msg
// };

///
/// 检查是否 合规
/// 检查是否 变量
///
pub fn is_var(charlist: &[String], istop: bool, locationmsg: String) -> Result<bool, String> {
  // 变量片段中 含有换行
  if charlist.is_empty() {
    return Err(format!("var token word is empty,{}", locationmsg));
  }
  if charlist
    .iter()
    .filter(|&x| x.as_str() == "\n" || x.as_str() == "\r")
    .count()
    > 0
  {
    return Err(format!(
      r#"token word has contains "\n","\r",{} "#,
      locationmsg
    ));
  }
  // 变量类似 ;; || @a:10px;;
  if charlist[0].as_str() == ";" {
    return Err(format!(r#"token word is only semicolon,{} "#, locationmsg));
  }
  if istop {
    // 变量片段中首位必须是 @
    if charlist[0].as_str() != "@" {
      return Err(format!(
        r#"token word is not with @ begin,{} "#,
        locationmsg
      ));
    }
    // 先判断 是否含有 @import
    if charlist.join("").indexOf("@import", None) > -1 {
      return Ok(false);
    }
    // 判断是否复合基本格式
    if charlist.join("").split(':').count() != 2 {
      return Err(format!(
        r#"var token is not liek '@var: 10px',{} ,{}"#,
        charlist.join(""),
        locationmsg
      ));
    }
  }
  Ok(true)
}
