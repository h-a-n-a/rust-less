use crate::extend::vec_str::VecStrExtend;
use crate::new_less::comment::Comment;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::{FileInfo, FileWeakRef};
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{NodeRef, NodeWeakRef, StyleNode, VarRuleNode};
use crate::new_less::option::OptionExtend;
use crate::new_less::rule::Rule;
use crate::new_less::select_node::SelectorNode;
use crate::new_less::style_rule::StyleRuleNode;
use crate::new_less::var::Var;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub struct RuleNode {
  // 选择器 文字
  pub selector: Option<SelectorNode>,
  // 根据 原始内容 -> 转化的 字符数组
  pub origin_charlist: Vec<char>,
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
  // 全局上下文
  pub context: ParseContext,
}

impl Serialize for RuleNode {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("RuleNode", 4)?;
    state.serialize_field("content", &self.origin_charlist.poly())?;
    state.serialize_field("loc", &self.loc)?;
    state.serialize_field("select", &self.selector.as_ref().unwrap().value())?;
    state.serialize_field("block_node", &self.block_node)?;
    state.end()
  }
}

impl Debug for RuleNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("RuleNode")
      .field("content", &self.origin_charlist.poly())
      .field("loc", &self.loc)
      .field("select", &self.selector.as_ref().unwrap().value())
      .field("block_node", &self.block_node)
      .finish()
  }
}

impl RuleNode {
  ///
  /// 构造方法
  ///
  pub fn new(
    charlist: Vec<char>,
    selector_txt: Vec<char>,
    loc: Option<Loc>,
    file_info: FileWeakRef,
    context: ParseContext,
  ) -> Result<NodeRef, String> {
    let mut change_loc: Option<Loc> = loc.clone();
    let obj = RuleNode {
      selector: None,
      origin_charlist: charlist,
      loc,
      locmap: None,
      block_node: vec![],
      parent: None,
      weak_self: None,
      file_info,
      context,
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
    if heapobj.deref().borrow().get_options().sourcemap {
      let (calcmap, _) = LocMap::merge(
        change_loc.as_ref().unwrap(),
        &heapobj.borrow().origin_charlist,
      );
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

  pub fn visit_mut_file(&self, fileinfo: &mut FileInfo) {
    self.block_node.iter().for_each(|x| {
      if let StyleNode::Rule(rule) = x {
        rule.borrow().visit_mut_file(fileinfo);
      }
    });
  }

  pub fn getrules(&self) -> Vec<NodeRef> {
    let mut list = vec![];

    self.block_node.iter().for_each(|x| {
      if let StyleNode::Rule(rule) = x {
        list.push(rule.clone());
      }
    });
    list
  }

  pub fn get_style_rule(&self) -> Vec<StyleRuleNode> {
    let mut list = vec![];
    self.block_node.iter().for_each(|x| {
      if let StyleNode::Var(VarRuleNode::StyleRule(style)) = x {
        list.push(style.clone());
      }
    });
    list
  }

  pub fn parse_heap(obj: NodeRef) -> Result<(), String> {
    obj.deref().borrow_mut().parse_comment()?;
    obj.deref().borrow_mut().parse_var()?;
    obj.deref().borrow_mut().parse_rule()?;
    Ok(())
  }

  pub fn code_gen(&self, content: &mut String) -> Result<(), String> {
    let rules = self.get_style_rule();

    if !rules.is_empty() {
      let (select_txt, media_txt) = self.selector.as_ref().unwrap().code_gen().unwrap();

      let mut tab: String = "".to_string();
      let mut index = 0;
      while index < self.get_options().tabspaces {
        tab += " ";
        index += 1;
      }

      let create_rules = |tab: String| -> Result<String, String> {
        let mut res: String = "".to_string();
        for rule_res in rules {
          res += &format!("{}{}{}", tab.clone(), rule_res.code_gen()?, "\n");
        }
        Ok(res)
      };

      if media_txt.is_empty() {
        *content += format!("\n{}{}\n{}\n{}\n", select_txt, "{", create_rules(tab)?, "}").as_ref();
      } else {
        *content += format!(
          "\n{}{}\n{}{}\n{}\n{}\n{}",
          media_txt,
          "{",
          tab.clone() + &select_txt,
          "{",
          create_rules(tab.clone() + &tab.clone())?,
          "  }",
          "}"
        )
        .as_ref();
      }
    }

    for node_ref in self.getrules() {
      node_ref.deref().borrow().code_gen(content)?;
    }

    Ok(())
  }
}
