use crate::new_less::comment::CommentNode;
use crate::new_less::import::ImportNode;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::media::MediaQuery;
use crate::new_less::option::OptionExtend;
use crate::new_less::parse::{RuleNode, RuleNodeJson};
use crate::new_less::select::Selector;
use crate::new_less::style_rule::StyleRuleNode;
use crate::new_less::var_node::VarNode;
use serde::Serialize;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub type NodeWeakRef = Option<Weak<RefCell<RuleNode>>>;
pub type NodeRef = Rc<RefCell<RuleNode>>;

#[derive(Debug, Clone)]
pub enum StyleNode {
  Comment(CommentNode),
  Var(VarRuleNode),
  Rule(NodeRef),
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
  pub fn new(txt: String, loc: &mut Option<Loc>, parent: NodeWeakRef) -> Result<Self, String> {
    let mut map: Option<LocMap> = None;
    match parent.unwrap().upgrade() {
      None => {}
      Some(p) => {
        if p.deref().borrow().get_options().sourcemap {
          let (calcmap, end) = LocMap::merge(loc.as_ref().unwrap(), &txt);
          *loc = Some(end);
          map = Some(calcmap);
        }
      }
    }
    // 处理 media
    match MediaQuery::new(txt.clone(), loc.clone(), map.clone()) {
      HandleResult::Success(obj) => {
        return Ok(SelectorNode::Media(obj));
      }
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    // 处理 select
    match Selector::new(txt.clone(), loc.clone(), map) {
      HandleResult::Success(obj) => {
        return Ok(SelectorNode::Select(obj));
      }
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    Err(format!("nothing node match the txt -> {}", txt))
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
  StyleRule(StyleRuleNode),
}

///
/// 联合 节点 声明
///
impl VarRuleNode {
  ///
  /// 初始化
  ///
  pub fn new(txt: String, loc: Option<Loc>, parent: NodeWeakRef) -> Result<Self, String> {
    // 处理 导入
    match ImportNode::new(txt.clone(), loc.clone(), parent.clone()) {
      HandleResult::Success(obj) => return Ok(VarRuleNode::Import(obj)),
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    // 处理 变量声明
    match VarNode::new(txt.clone(), loc.clone(), parent.clone()) {
      HandleResult::Success(obj) => return Ok(VarRuleNode::Var(obj)),
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    // 处理 规则
    match StyleRuleNode::new(txt.clone(), loc, parent) {
      HandleResult::Success(obj) => return Ok(VarRuleNode::StyleRule(obj)),
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    Err(format!("nothing node match the txt -> {}", txt))
  }
}
