use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::media::MediaQuery;
use crate::new_less::node::{HandleResult, NodeWeakRef};
use crate::new_less::option::OptionExtend;
use crate::new_less::select::Selector;
use serde::Serialize;
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

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
    match parent.as_ref().unwrap().upgrade() {
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
    match MediaQuery::new(txt.clone(), loc.clone(), map.clone(), parent.clone()) {
      HandleResult::Success(obj) => {
        return Ok(SelectorNode::Media(obj));
      }
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    // 处理 select
    match Selector::new(txt.clone(), loc.clone(), map, parent.clone()) {
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

  ///
  /// 递归收集方法 向上查找
  /// 如果是 media 就放在 第一个 0 位置 数组中
  /// 如果是 select 就放在 第二个 1 位置  数组中
  ///
  fn collect(&self, mut tuple: &mut (&mut Vec<String>, &mut Vec<Vec<String>>)) {
    match self {
      SelectorNode::Select(select) => {
        tuple.deref_mut().1.push(select.single_select_txt.clone());
        let rule = select.parent.as_ref().unwrap().upgrade().unwrap();
        if rule.deref().borrow().parent.is_some() {
          let parent_rule = rule.deref().borrow().parent.as_ref().unwrap().upgrade().unwrap();
          parent_rule.deref().borrow().selector.as_ref().unwrap().collect(tuple);
        }
      }
      SelectorNode::Media(media) => {
        tuple.0.push(media.origin_txt.clone());
        let rule = media.parent.as_ref().unwrap().upgrade().unwrap();
        if rule.deref().borrow().parent.is_some() {
          let parent_rule = rule.deref().borrow().parent.as_ref().unwrap().upgrade().unwrap();
          parent_rule.deref().borrow().selector.as_ref().unwrap().collect(tuple);
        }
      }
    };
  }

  pub fn code_gen(&self) -> Result<String, String> {
    let mut media_rules: Vec<String> = vec![];
    let mut select_rules: Vec<Vec<String>> = vec![];
    let mut tuple = (&mut media_rules, &mut select_rules);
    self.collect(&mut tuple);
    Ok("".to_string())
  }
}
