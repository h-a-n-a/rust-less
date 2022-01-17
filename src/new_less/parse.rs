use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use crate::extend::string::StringExtend;
use crate::new_less::comment::Comment;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{StyleNode, StyleNodeJson};
use crate::new_less::option::ParseOption;
use crate::new_less::rule::Rule;
use crate::new_less::var::Var;
use serde::{Serialize};
use crate::new_less::select::Selector;

#[derive(Debug, Clone)]
pub struct RuleNode {
  // 节点内容
  pub content: String,
  // 选择器 文字
  pub selector: Selector,
  // 根据 原始内容 -> 转化的 字符数组
  pub origin_charlist: Vec<String>,
  // 节点坐标
  pub loc: Loc,
  // 当前所有 索引 对应的 坐标行列 -> 用于执行 sourcemap
  pub locmap: Option<LocMap>,
  // 内部调用方式时 需要拿到对应的 转化配置
  pub option: ParseOption,
  // 节点 父节点
  pub parent: Option<Weak<RefCell<RuleNode>>>,
  // 节点 子节点
  pub block_node: Vec<StyleNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuleNodeJson {
  // 节点内容
  pub content: String,
  // 选择器 文字
  pub selector_txt: String,
  // 节点坐标
  pub loc: Loc,
  // 节点 子节点
  pub block_node: Vec<StyleNodeJson>,
}

impl RuleNode {
  ///
  /// 转 json 标准化
  ///
  pub fn tojson(&self) -> RuleNodeJson {
    let mut block_node = vec![];
    self.block_node.clone().into_iter().for_each(|node| {
      match node {
        StyleNode::Comment(cc) => {
          block_node.push(StyleNodeJson::Comment(cc));
        }
        StyleNode::Var(vv) => {
          block_node.push(StyleNodeJson::Var(vv));
        }
        StyleNode::Rule(rule) => {
          let futex_rule = rule.deref().borrow().deref().clone().tojson();
          block_node.push(StyleNodeJson::Rule(futex_rule));
        }
      }
    });
    RuleNodeJson {
      selector_txt: self.selector.value(),
      content: self.content.clone(),
      loc: self.loc.clone(),
      block_node,
    }
  }
  
  ///
  /// 构造方法
  ///
  pub fn new(content: String, selector_txt: String, loc: Loc, option: ParseOption, parent: Option<Weak<RefCell<RuleNode>>>) -> Result<Rc<RefCell<RuleNode>>, String> {
    let origin_charlist = content.tocharlist();
    let mut locmap: Option<LocMap> = None;
    if option.sourcemap {
      locmap = Some(LocMap::new(content.to_string()));
    }
    let selector = Selector::new(selector_txt);
    let obj = RuleNode {
      content,
      selector,
      origin_charlist,
      loc,
      locmap,
      option,
      parent,
      block_node: vec![],
    };
    // Ok(Rc::new(RefCell::new(obj)))
    match obj.parse() {
      Ok(obj) => {
        Ok(obj)
      }
      Err(msg) => {
        Err(msg)
      }
    }
  }
  
  pub fn parse(mut self) -> Result<Rc<RefCell<RuleNode>>, String> {
    match self.parse_comment() {
      Ok(blocks) => {
        let mut enum_cc = blocks.into_iter().map(|x| {
          StyleNode::Comment(x)
        }).collect::<Vec<StyleNode>>();
        self.block_node.append(&mut enum_cc);
      }
      Err(msg) => {
        return Err(msg);
      }
    }
    match self.parse_var() {
      Ok(blocks) => {
        let mut enum_var = blocks.into_iter().map(|x| {
          StyleNode::Var(x)
        }).collect::<Vec<StyleNode>>();
        self.block_node.append(&mut enum_var);
      }
      Err(msg) => {
        return Err(msg);
      }
    }
    let parent = Rc::new(RefCell::new(self));
    let mut enum_rule = match parent.borrow_mut().parse_rule() {
      Ok(blocks) => {
        for node in blocks.clone() {
          node.borrow_mut().parent = Some(Rc::downgrade(&parent));
        }
        blocks.into_iter().map(
          |x| {
            StyleNode::Rule(x)
          })
          .collect::<Vec<StyleNode>>()
      }
      Err(msg) => {
        return Err(msg);
      }
    };
    parent.borrow_mut().block_node.append(&mut enum_rule);
    Ok(parent)
  }
}