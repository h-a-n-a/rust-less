use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::media::MediaQuery;
use crate::new_less::node::NodeWeakRef;
use crate::new_less::option::OptionExtend;
use crate::new_less::select::Selector;
use crate::new_less::var::HandleResult;
use serde::Serialize;
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
  pub fn new(
    charlist: Vec<char>,
    loc: &mut Option<Loc>,
    parent: NodeWeakRef,
  ) -> Result<Self, String> {
    let mut map: Option<LocMap> = None;
    match parent.as_ref().unwrap().upgrade() {
      None => {}
      Some(p) => {
        if p.deref().borrow().get_options().sourcemap {
          let (calcmap, end) = LocMap::merge(loc.as_ref().unwrap(), &charlist);
          *loc = Some(end);
          map = Some(calcmap);
        }
      }
    }
    // 处理 media
    match MediaQuery::new(charlist.clone(), loc.clone(), map.clone(), parent.clone()) {
      HandleResult::Success(obj) => {
        return Ok(SelectorNode::Media(obj));
      }
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    // 处理 select
    match Selector::new(charlist.clone(), loc.clone(), map, parent) {
      HandleResult::Success(obj) => {
        return Ok(SelectorNode::Select(obj));
      }
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };
    Err(format!("nothing node match the txt -> {}", charlist.poly()))
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
          let parent_rule = rule
            .deref()
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap();
          parent_rule
            .deref()
            .borrow()
            .selector
            .as_ref()
            .unwrap()
            .collect(tuple);
        }
      }
      SelectorNode::Media(media) => {
        tuple.0.push(media.charlist.poly());
        let rule = media.parent.as_ref().unwrap().upgrade().unwrap();
        if rule.deref().borrow().parent.is_some() {
          let parent_rule = rule
            .deref()
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap();
          parent_rule
            .deref()
            .borrow()
            .selector
            .as_ref()
            .unwrap()
            .collect(tuple);
        }
      }
    };
  }

  ///
  /// 代码生成方法
  ///
  pub fn code_gen(&self) -> Result<(String, String), String> {
    let mut media_rules: Vec<String> = vec![];
    let mut select_rules: Vec<Vec<String>> = vec![];
    let mut tuple = (&mut media_rules, &mut select_rules);
    self.collect(&mut tuple);

    // 处理收集的 select 字符串
    let mut select_txt: Vec<String> = vec![];
    select_rules.reverse();
    for word_groups in select_rules {
      if select_txt.is_empty() {
        select_txt.append(
          &mut word_groups
            .iter()
            .map(|x| x.replace("$(&)", ""))
            .collect::<Vec<String>>(),
        );
      } else {
        let mut new_list: Vec<String> = vec![];
        // 交叉相乘
        for origin in select_txt {
          for item in &word_groups {
            if item.indexOf("$(&)", None) > -1 {
              new_list.push(item.replace("$(&)", &origin).to_string());
            } else {
              new_list.push(format!("{} {}", origin, item));
            }
          }
        }
        select_txt = new_list;
      }
    }

    // 处理收集的 media 字符串
    let mut media_txt: Vec<String> = vec![];
    media_rules.reverse();
    for media_word in media_rules {
      if media_txt.is_empty() {
        media_txt.push(media_word);
      } else {
        media_txt.push(media_word[6..].to_string())
      }
    }
    let res = (select_txt.join(","), media_txt.join(" and "));
    Ok(res)
  }
}
