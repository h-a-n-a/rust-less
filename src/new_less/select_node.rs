use crate::extend::vec_str::VecCharExtend;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::media::MediaQuery;
use crate::new_less::node::NodeWeakRef;
use crate::new_less::option::OptionExtend;
use crate::new_less::var::HandleResult;
use serde::Serialize;
use std::ops::Deref;
use crate::new_less::new_select::NewSelector;

#[derive(Debug, Clone, Serialize)]
pub enum SelectorNode {
  Select(NewSelector),
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
    match NewSelector::new(charlist.clone(), loc.clone(), map, parent) {
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
  /// 代码生成方法
  ///
  pub fn code_gen(&self) -> Result<(String, String), String> {
    // 处理收集的 select 字符串
    let select_txt: Vec<String> = vec![];
    // 处理收集的 media 字符串
    let media_txt: Vec<String> = vec![];

    // media_rules.reverse();
    // for media_word in media_rules {
    //   if media_txt.is_empty() {
    //     media_txt.push(media_word);
    //   } else {
    //     media_txt.push(media_word[6..].to_string())
    //   }
    // }
    let res = (select_txt.join(","), media_txt.join(" and "));
    Ok(res)
  }
}
