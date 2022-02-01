use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::media::MediaQuery;
use crate::new_less::node::{HandleResult, NodeWeakRef};
use crate::new_less::option::OptionExtend;
use crate::new_less::select::Selector;
use serde::Serialize;
use std::ops::Deref;

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

  fn code_gen(&self) -> Result<String, String> {
    let txt: Vec<String> = vec![];
    match self {
      SelectorNode::Select(select) => {
        let list = select
          .single_select_txt
          .iter()
          .map(|x| x.clone())
          .collect::<Vec<String>>();
      }
      SelectorNode::Media(media) => {}
    }

    Ok("".to_string())
  }
}
