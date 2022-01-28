use crate::new_less::comment::CommentNode;
use crate::new_less::import::ImportNode;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::media::MediaQuery;
use crate::new_less::parse::{RuleNode, RuleNodeJson};
use crate::new_less::select::Selector;
use crate::new_less::style_rule::StyleRuleNode;
use crate::new_less::var_node::VarNode;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum StyleNode {
  Comment(CommentNode),
  Var(VarRuleNode),
  Rule(Rc<RefCell<RuleNode>>),
}

#[derive(Debug, Clone, Serialize)]
pub enum StyleNodeJson {
  Comment(CommentNode),
  Var(VarRuleNode),
  Rule(RuleNodeJson),
}

#[derive(Debug, Clone, Serialize)]
pub enum SelectorNode {
  Select(Selector),
  Media(MediaQuery),
}

///
/// 创建 选择器 混合节点
///
impl SelectorNode {
  ///
  /// 初始化方法
  ///
  pub fn new(txt: String, loc: Option<Loc>, map: Option<LocMap>) -> Result<Self, String> {
    let mut msg: String = "".to_string();

    match MediaQuery::new(txt.clone(), loc.clone(), map.clone()) {
      Ok(obj) => {
        return Ok(SelectorNode::Media(obj));
      }
      Err(media_msg) => msg += &media_msg,
    };
    if msg == "select_txt not match media query" {
      // 确定是因为不适配 media 然后重新计算 select
      match Selector::new(txt, loc, map) {
        Ok(obj) => {
          return Ok(SelectorNode::Select(obj));
        }
        Err(select_msg) => msg += &select_msg,
      };
    }
    Err(msg)
  }

  pub fn value(&self) -> String {
    match self {
      SelectorNode::Select(obj) => obj.value(),
      SelectorNode::Media(obj) => obj.value(),
    }
  }
}

///
/// 处理类型
///
pub enum HandleResult<T> {
  /// 匹配成功 且 处理成功
  Success(T),

  /// 匹配成功 且 处理失败
  Fail(String),

  /// 匹配失败
  Swtich,
}

///
/// 变量内容
///
#[derive(Debug, Clone, Serialize)]
pub enum VarRuleNode {
  /// 引用
  Import(ImportNode),

  /// 变量声明
  Var(VarNode),

  /// 样式规则
  Rule(StyleRuleNode),
}

///
/// 联合 节点 声明
///
impl VarRuleNode {
  pub fn new(txt: String, loc: Option<Loc>) -> Result<Self, String> {
    // 处理 导入
    match ImportNode::new(txt.clone(), loc.clone()) {
      HandleResult::Success(obj) => return Ok(VarRuleNode::Import(obj)),
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    // 处理 变量声明
    match VarNode::new(txt.clone(), loc.clone()) {
      HandleResult::Success(obj) => return Ok(VarRuleNode::Var(obj)),
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    // 处理 规则
    match StyleRuleNode::new(txt.clone(), loc) {
      HandleResult::Success(obj) => return Ok(VarRuleNode::Rule(obj)),
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    Err(format!("nothing node match the txt -> {}", txt))
  }
}
