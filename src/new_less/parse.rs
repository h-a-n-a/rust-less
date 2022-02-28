use crate::extend::string::StringExtend;
use crate::new_less::comment::Comment;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::{FileInfo, FileWeakRef};
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{NodeRef, NodeWeakRef, StyleNode, StyleNodeJson, VarRuleNode};
use crate::new_less::option::OptionExtend;
use crate::new_less::rule::Rule;
use crate::new_less::select_node::SelectorNode;
use crate::new_less::style_rule::StyleRuleNode;
use crate::new_less::var::Var;
use derivative::Derivative;
use serde::Serialize;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Derivative)]
#[derivative(Debug, Clone)]
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
  #[derivative(Debug = "ignore")]
  pub parent: NodeWeakRef,
  // 自己的引用关系
  #[derivative(Debug = "ignore")]
  pub weak_self: NodeWeakRef,
  // 节点 子节点
  pub block_node: Vec<StyleNode>,
  // 文件弱引用
  #[derivative(Debug = "ignore")]
  pub file_info: FileWeakRef,
  // 全局上下文
  #[derivative(Debug = "ignore")]
  pub context: ParseContext,
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
    context: ParseContext,
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
