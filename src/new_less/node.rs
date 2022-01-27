use crate::new_less::comment::CommentNode;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::media::MediaQuery;
use crate::new_less::parse::{RuleNode, RuleNodeJson};
use crate::new_less::select::Selector;
use crate::new_less::var::VarNode;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum StyleNode {
    Comment(CommentNode),
    Var(VarNode),
    Rule(Rc<RefCell<RuleNode>>),
}

#[derive(Debug, Clone, Serialize)]
pub enum StyleNodeJson {
    Comment(CommentNode),
    Var(VarNode),
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
    pub fn new(txt: String, loc: Option<Loc>, map: Option<LocMap>) -> Result<Self, String> {
        let mut msg: String = "".to_string();
        match MediaQuery::new(txt.clone(), loc, map) {
            Ok(obj) => {
                return Ok(SelectorNode::Media(obj));
            }
            Err(media_msg) => msg += &media_msg,
        };
        if msg == "select_txt not match media query" {
            // 确定是因为不适配 media 然后重新计算 select
            match Selector::new(txt) {
                Ok(obj) => {
                    return Ok(SelectorNode::Select(obj));
                }
                Err(select_msg) => msg += &select_msg,
            };
        }
        Err(msg)
    }

    pub fn value(&self) -> String {
        match self {
            SelectorNode::Select(obj) => obj.value(),
            SelectorNode::Media(obj) => obj.value(),
        }
    }
}
