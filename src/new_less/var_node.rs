use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;
use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{HandleResult, NodeWeakRef};
use crate::new_less::option::ParseOption;
use crate::new_less::scan::{traversal, ScanArg, ScanResult};
use crate::new_less::token::lib::Token;
use crate::new_less::token::var::{TokenVarKeyAllow, TokenVarValue};
use serde::Serialize;
use std::ops::Deref;
use crate::new_less::fileinfo::FileWeakRef;

#[derive(Debug, Clone, Serialize)]
pub struct VarNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Option<Loc>,

  // 内部处理 地图
  #[serde(skip_serializing)]
  map: LocMap,

  // 字符串 操作 序列
  #[serde(skip_serializing)]
  charlist: Vec<String>,

  // 节点 父节点
  #[serde(skip_serializing)]
  pub parent: NodeWeakRef,

  // 文件信息
  #[serde(skip_serializing)]
  pub fileinfo: FileWeakRef,

  pub key: Option<String>,

  pub value: Option<String>,
}

impl VarNode {
  ///
  /// 初始化
  ///
  pub fn new(txt: String, loc: Option<Loc>, parent: NodeWeakRef, fileinfo: FileWeakRef) -> HandleResult<Self> {
    let map = if loc.is_none() {
      LocMap::new(txt.clone())
    } else {
      LocMap::merge(loc.as_ref().unwrap(), &txt).0
    };
    let mut obj = Self {
      content: txt.clone(),
      loc,
      map,
      charlist: txt.tocharlist(),
      parent,
      fileinfo,
      key: None,
      value: None,
    };
    if !obj.content.is_empty() && obj.charlist.get(0).unwrap() != "@" && !obj.is_top() {
      return HandleResult::Swtich;
    } else if !obj.content.is_empty() && obj.charlist.get(0).unwrap() != "@" && obj.is_top() {
      return HandleResult::Fail(obj.error_msg(&0));
    } else if obj.content.is_empty() {
      return HandleResult::Fail("var declare txt is empty!".to_string());
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
  /// 获取选项
  ///
  pub fn get_options(&self) -> ParseOption {
    match self.fileinfo.clone() {
      None => Default::default(),
      Some(file) => {
        file.upgrade().unwrap().deref().borrow().option.clone()
      }
    }
  }

  ///
  /// 报错信息
  ///
  pub fn error_msg(&self, index: &usize) -> String {
    let error_loc = self.map.get(index).unwrap();
    let char = self.charlist.get(*index).unwrap().to_string();
    format!(
      "text {}, char {} is not allow, line is {} col is {}",
      &self.content, char, error_loc.line, error_loc.col
    )
  }

  pub fn parse_var_ident(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let mut hasspace = false;
    match traversal(
      Some(*start),
      charlist,
      &mut (move |arg, charword| {
        let ScanArg {
          mut temp,
          index,
          mut hasend,
        } = arg;
        let (_, char, next) = charword;
        if hasspace && Token::is_space_token(&next) {
          return Ok(ScanResult::Skip);
        } else if hasspace && !Token::is_space_token(&char) {
          if char == TokenVarKeyAllow::Colon.tostr_value() {
            temp += &char;
          } else {
            return Err(self.error_msg(&(index - 1)));
          }
        } else if Token::is_token(&char) && !hasspace {
          if TokenVarKeyAllow::is(&char) {
            if char == TokenVarKeyAllow::Colon.tostr_value() {
              hasend = true;
            } else {
              temp += &char;
            }
          } else if Token::is_space_token(&char) {
            hasspace = true;
            temp += &char;
          } else {
            return Err(self.error_msg(&index));
          }
        } else if !Token::is_token(&char) && !hasspace {
          temp += &char;
        }

        let new_arg = ScanArg {
          index,
          temp,
          hasend,
        };
        Ok(ScanResult::Arg(new_arg))
      }),
    ) {
      Ok(obj) => Ok(obj),
      Err(msg) => Err(msg),
    }
  }

  pub fn parse_var_value(&self, start: &usize) -> Result<(String, usize), String> {
    // 取分号前一位 最后一定是分号
    let end = self.charlist.len() - 1;
    let c = self.charlist[*start..end].poly().trim().to_string();
    let charlist = &c.tocharlist();
    match traversal(
      None,
      charlist,
      &mut (|arg, charword| {
        let ScanArg {
          mut temp,
          index,
          hasend,
        } = arg;
        let (_, char, _) = charword;
        //todo! 目前是简单计算 后续需要修改加强 value 的 parse 过程
        if Token::is_token(&char) {
          if TokenVarValue::is(&char) {
            temp += &char;
          } else {
            return Err(self.error_msg(&(index - 1)));
          }
        } else {
          temp += &char;
        }
        let new_arg = ScanArg {
          index,
          temp,
          hasend,
        };
        Ok(ScanResult::Arg(new_arg))
      }),
    ) {
      Ok(obj) => Ok((obj.0, self.charlist.len() - 1)),
      Err(msg) => Err(msg),
    }
  }

  ///
  /// 转化校验
  ///
  fn parse(&mut self) -> Result<(), String> {
    let charlist = &self.charlist.clone();
    if charlist.is_empty() {
      return Err("var declare text is empty".to_string());
    }
    let index = 1;
    match traversal(
      Some(index),
      charlist,
      &mut (|arg, _| {
        let mut index = arg.index;
        if self.key.is_none() {
          let (key, jump) = self.parse_var_ident(&arg.index)?;
          index = jump;
          self.key = Some("@".to_string() + &key);
        } else if self.value.is_none() {
          let (value, jump) = self.parse_var_value(&arg.index)?;
          index = jump;
          self.value = Some(value);
        } else if self.value.is_some() && self.key.is_some() {
          return Err(self.error_msg(&index));
        }
        let new_arg = ScanArg {
          index,
          temp: arg.temp,
          hasend: false,
        };
        Ok(ScanResult::Arg(new_arg))
      }),
    ) {
      Ok(_) => {}
      Err(msg) => {
        return Err(msg);
      }
    };

    Ok(())
  }
}
