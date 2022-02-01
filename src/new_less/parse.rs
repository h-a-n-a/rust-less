use crate::extend::string::StringExtend;
use crate::new_less::comment::Comment;
use crate::new_less::fileinfo::FileWeakRef;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{NodeRef, NodeWeakRef, StyleNode, StyleNodeJson, VarRuleNode};
use crate::new_less::option::OptionExtend;
use crate::new_less::rule::Rule;
use crate::new_less::select_node::SelectorNode;
use crate::new_less::style_rule::StyleRuleNode;
use crate::new_less::var::Var;
use serde::Serialize;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct RuleNode {
  // 节点内容
  pub content: String,
  // 选择器 文字
  pub selector: Option<SelectorNode>,
  // 根据 原始内容 -> 转化的 字符数组
  pub origin_charlist: Vec<String>,
  // 节点坐标
  pub loc: Option<Loc>,
  // 当前所有 索引 对应的 坐标行列 -> 用于执行 sourcemap
  pub locmap: Option<LocMap>,
  // 节点 父节点
  pub parent: NodeWeakRef,
  // 自己的引用关系
  pub weak_self: NodeWeakRef,
  // 节点 子节点
  pub block_node: Vec<StyleNode>,
  // 文件弱引用
  pub file_info: FileWeakRef,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuleNodeJson {
  // 节点内容
  pub content: String,
  // 选择器 文字
  pub selector_txt: String,
  // 节点坐标
  pub loc: Option<Loc>,
  // 节点 子节点
  pub block_node: Vec<StyleNodeJson>,
}

impl RuleNode {
  ///
  /// 转 json 标准化
  ///
  pub fn tojson(&self) -> RuleNodeJson {
    let mut block_node = vec![];
    self
      .block_node
      .clone()
      .into_iter()
      .for_each(|node| match node {
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
      });
    RuleNodeJson {
      selector_txt: self.selector.as_ref().unwrap().value(),
      content: self.content.clone(),
      loc: self.loc.as_ref().cloned(),
      block_node,
    }
  }

  ///
  /// 构造方法
  ///
  pub fn new(
    content: String,
    selector_txt: String,
    loc: Option<Loc>,
    file_info: FileWeakRef,
  ) -> Result<NodeRef, String> {
    let origin_charlist = content.tocharlist();
    let mut change_loc: Option<Loc> = loc.clone();
    let obj = RuleNode {
      content: content.clone(),
      selector: None,
      origin_charlist,
      loc,
      locmap: None,
      block_node: vec![],
      parent: None,
      weak_self: None,
      file_info,
    };
    let heapobj = Rc::new(RefCell::new(obj));
    let wek_self = Rc::downgrade(&heapobj);
    heapobj.borrow_mut().weak_self = Some(wek_self.clone());

    let selector = match SelectorNode::new(selector_txt, &mut change_loc, Some(wek_self)) {
      Ok(result) => result,
      Err(msg) => {
        return Err(msg);
      }
    };
    heapobj.borrow_mut().selector = Some(selector);
    if heapobj.borrow().get_options().sourcemap {
      let (calcmap, _) = LocMap::merge(change_loc.as_ref().unwrap(), &content);
      heapobj.borrow_mut().locmap = Some(calcmap);
    }

    match Self::parse_heap(heapobj.clone()) {
      Ok(_) => {}
      Err(msg) => {
        return Err(msg);
      }
    }
    Ok(heapobj)
  }

  pub fn getrules(&self) -> Vec<NodeRef> {
    let mut list = vec![];
    self.block_node.iter().for_each(|x| match x {
      StyleNode::Rule(rule) => list.push(rule.clone()),
      _ => {}
    });
    list
  }

  pub fn get_style_rule(&self) -> Vec<StyleRuleNode> {
    let mut list = vec![];
    self.block_node.iter().for_each(|x| match x {
      StyleNode::Var(var) => match var {
        VarRuleNode::StyleRule(style) => list.push(style.clone()),
        _ => {}
      },
      _ => {}
    });
    list
  }

  pub fn parse_heap(obj: NodeRef) -> Result<(), String> {
    let mut comments = match obj.borrow().parse_comment() {
      Ok(blocks) => blocks
        .into_iter()
        .map(StyleNode::Comment)
        .collect::<Vec<StyleNode>>(),
      Err(msg) => {
        return Err(msg);
      }
    };
    obj.borrow_mut().block_node.append(&mut comments);
    let mut vars = match obj.borrow().parse_var() {
      Ok(blocks) => blocks
        .into_iter()
        .map(StyleNode::Var)
        .collect::<Vec<StyleNode>>(),
      Err(msg) => {
        return Err(msg);
      }
    };
    obj.borrow_mut().block_node.append(&mut vars);
    let mut enum_rule = match obj.borrow().parse_rule() {
      Ok(blocks) => {
        for node in blocks.clone() {
          let mut node_value = node.borrow_mut();
          node_value.parent = Some(Rc::downgrade(&obj));
        }
        blocks
          .into_iter()
          .map(StyleNode::Rule)
          .collect::<Vec<StyleNode>>()
      }
      Err(msg) => {
        return Err(msg);
      }
    };
    obj.borrow_mut().block_node.append(&mut enum_rule);
    Ok(())
  }

  pub fn code_gen(&self, content: &mut String) {
    let select_txt = match self.selector.as_ref().unwrap() {
      SelectorNode::Select(se) => se.origin_txt.clone(),
      SelectorNode::Media(me) => me.origin_txt.clone(),
    };
    let txt = self.selector.as_ref().unwrap().code_gen().unwrap();
    println!("{}", txt);

    let stylerules = self
      .get_style_rule()
      .iter()
      .map(|x| x.content.clone())
      .collect::<Vec<String>>()
      .join("\n");

    *content += format!("\n{}{}\n{}\n{}", select_txt, "{", stylerules, "}").as_ref();

    self.getrules().iter().for_each(|x| {
      x.deref().borrow().code_gen(content);
    });
  }
}
