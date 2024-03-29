use crate::new_less::fileinfo::FileWeakRef;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::media::MediaQuery;
use crate::new_less::node::NodeWeakRef;
use crate::new_less::option::OptionExtend;
use crate::new_less::select::NewSelector;
use crate::new_less::var::HandleResult;
use serde::Serialize;
use std::ops::Deref;
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value")]
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
    fileinfo: FileWeakRef,
  ) -> Result<Self, String> {
    let mut map: Option<LocMap> = None;
    let start_loc = loc.as_ref().cloned();
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
    match MediaQuery::new(charlist.clone(), start_loc.clone(), map.clone(), parent.clone()) {
      HandleResult::Success(obj) => {
        return Ok(SelectorNode::Media(obj));
      }
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };

    let obj = NewSelector::new(charlist.clone(), start_loc.clone(), map, parent, fileinfo);
    Ok(SelectorNode::Select(obj))
  }

  ///
  /// 反序列化
  ///
  pub fn deserializer(map: &Map<String, Value>, parent: NodeWeakRef, fileinfo: FileWeakRef) -> Result<Self, String> {
    // 处理 select
    if parent.is_none() {
      return Err("SelectorNode -> parent must not be None!".to_string());
    }
    let value_type = map.get("type").unwrap().to_string();
    if value_type == r#""Select""# {
      // 处理引用
      let value_map = map.get("value").unwrap().as_object().unwrap();
      return Ok(SelectorNode::Select(NewSelector::deserializer(value_map, parent, fileinfo)?));
    } else if value_type == r#""Media""# {
      // 处理变量
      let value_map = map.get("value").unwrap().as_object().unwrap();
      return Ok(SelectorNode::Media(MediaQuery::deserializer(value_map, parent)?));
    }
    Err("SelectorNode -> noting type is matched".to_string())
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
    let mut node = None;
    if let Self::Select(ss) = &self {
      node = ss.parent.clone();
    } else if let Self::Media(ss) = &self {
      node = ss.parent.clone();
    }

    let nearly_select_node = NewSelector::find_up_select_node(node.clone());
    let nearly_media_node = MediaQuery::find_up_media_node(node.clone());

    if nearly_select_node.is_none() && nearly_media_node.is_none() {
      return Err("codegen select_node -> nearly_select_node || nearly_media_node  one of them is not empty!".to_string());
    }

    let mut select_res = "".to_string();
    if let Some(snode) = nearly_select_node {
      if let SelectorNode::Select(s) = snode.upgrade().unwrap().borrow().selector.as_ref().unwrap()
      {
        select_res = s.code_gen()?.join(",");
      }
    }

    let mut media_res = "".to_string();
    if let Some(mnode) = nearly_media_node {
      if let SelectorNode::Media(m) = mnode.upgrade().unwrap().borrow().selector.as_ref().unwrap() {
        media_res = m.code_gen().join(" and ");
      }
    }

    Ok((select_res, media_res))
  }
}
