use crate::extend::str_into::StringInto;
use crate::extend::string::StringExtend;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{HandleResult, NodeWeakRef};
use crate::new_less::scan::{traversal, ScanArg, ScanResult};
use crate::new_less::token::import::TokenImport;
use crate::new_less::token::lib::Token;
use serde::Serialize;

///
/// import 处理
///
#[derive(Debug, Clone, Serialize)]
pub struct ImportNode {
  // 原始字符
  #[serde(rename(serialize = "content"))]
  pub origin_txt: String,

  // 节点坐标
  pub loc: Option<Loc>,

  // 内部处理 地图
  #[serde(skip_serializing)]
  map: LocMap,

  // 自身 Rule 的弱引用
  #[serde(skip_serializing)]
  parent: NodeWeakRef,

  // 内部快速扫词 字符串 数组
  #[serde(skip_serializing)]
  charlist: Vec<String>,

  // 经常 插件 hook 的 计算完的 文件地址
  #[serde(rename(serialize = "path"))]
  parse_hook_url: String,
}

impl ImportNode {
  ///
  /// 初始化方法
  ///
  pub fn new(txt: String, loc: Option<Loc>, parent: NodeWeakRef) -> HandleResult<Self> {
    let map = if loc.is_none() {
      LocMap::new(txt.clone())
    } else {
      LocMap::merge(loc.as_ref().unwrap(), &txt).0
    };
    let obj = Self {
      origin_txt: txt.to_string(),
      loc,
      map,
      parent,
      charlist: txt.trim().to_string().tocharlist(),
      parse_hook_url: "".to_string(),
    };
    if obj.origin_txt.len() < 7 {
      return HandleResult::Swtich;
    } else if &obj.origin_txt[0..7] != "@import" {
      return HandleResult::Swtich;
    }
    match obj.parse() {
      Ok(_) => HandleResult::Success(obj),
      Err(msg) => HandleResult::Fail(msg),
    }
  }

  ///
  /// 判断是否是 顶层 节点 下的变量
  ///
  pub fn is_top(&self) -> bool {
    self.parent.is_none()
  }

  ///
  /// 报错信息
  ///
  pub fn error_msg(&self, index: &usize) -> String {
    let error_loc = self.map.get(index).unwrap();
    let char = self.charlist.get(*index).unwrap().to_string();
    format!(
      "text {}, char {} is not allow, line is {} col is {}",
      &self.origin_txt, char, error_loc.line, error_loc.col
    )
  }

  ///
  /// 解析 字符串
  ///
  fn parse(&self) -> Result<(), String> {
    let charlist = &self.charlist.clone();
    let index = 7;
    let mut has_apost = false;
    let mut has_quote = false;

    let path = match traversal(
      Some(index),
      charlist,
      &mut (|arg, (_, char, _)| {
        let ScanArg {
          index,
          mut temp,
          mut hasend,
        } = arg;

        if has_apost || has_quote {
          if Token::is_token(&char) {
            if (TokenImport::Apost.tostr_value() == char && has_apost)
              || (TokenImport::Quote.tostr_value() == char && has_quote)
            {
              if index != charlist.len() - 2 {
                return Err(self.error_msg(&index));
              } else {
                hasend = true
              }
            } else {
              temp += &char;
            }
          } else {
            temp += &char;
          }
        } else {
          if Token::is_token(&char) {
            if !Token::is_space_token(&char) {
              if TokenImport::Apost.tostr_value() == char {
                has_apost = true;
              } else if TokenImport::Quote.tostr_value() == char {
                has_quote = true;
              } else {
                return Err(self.error_msg(&index));
              }
            }
          } else {
            return Err(self.error_msg(&index));
          }
        }

        Ok(ScanResult::Arg(ScanArg {
          index,
          temp,
          hasend,
        }))
      }),
    ) {
      Ok(res) => res.0,
      Err(msg) => {
        return Err(msg);
      }
    };

    Ok(())
  }
}
