use std::rc::Rc;
use crate::extend::string::StringExtend;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::option::ParseOption;
use serde::{Serialize};
use crate::new_less::comment::Comment;
use crate::new_less::rule::Rule;
use crate::new_less::var::Var;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum OriginBlockType {
  Comment,
  StyleRule,
  Var,
  Import,
}

#[derive(Debug, Clone, Serialize)]
pub struct OriginBlock {
  // 节点类型
  pub block_type: OriginBlockType,
  // 节点内容
  pub content: String,
  // 根据 原始内容 -> 转化的 字符数组
  #[serde(skip_serializing)]
  pub origin_charlist: Vec<String>,
  // 节点坐标
  pub loc: Loc,
  // 当前所有 索引 对应的 坐标行列 -> 用于执行 sourcemap
  #[serde(skip_serializing)]
  pub locmap: Option<LocMap>,
  // 内部调用方式时 需要拿到对应的 转化配置
  #[serde(skip_serializing)]
  pub option: ParseOption,
  // 节点 父节点
  #[serde(skip_serializing)]
  pub parent: Option<Rc<OriginBlock>>,
  // 节点 子节点
  pub block_node: Vec<OriginBlock>,
}


impl OriginBlock {
  ///
  /// 构造方法
  ///
  pub fn new(block_type: OriginBlockType, content: String, loc: Loc, option: ParseOption, parent: Option<Rc<OriginBlock>>) -> OriginBlock {
    let origin_charlist = content.tocharlist();
    let mut locmap: Option<LocMap> = None;
    if option.sourcemap {
      locmap = Some(LocMap::new(content.to_string()));
    }
    let obj = OriginBlock {
      block_type,
      content,
      origin_charlist,
      loc,
      locmap,
      option,
      parent,
      block_node: vec![],
    };
    obj
  }

  ///
  /// 创建评论
  ///
  pub fn create_comment(content: String, loc: Loc, option: ParseOption) -> OriginBlock {
    OriginBlock::new(OriginBlockType::Comment, content, loc, option, None)
  }

  ///
  /// 创建子模块
  ///
  pub fn create_rule(content: String, loc: Loc, option: ParseOption, parent: Option<Rc<OriginBlock>>) -> Result<OriginBlock, String> {
    let mut obj = OriginBlock::new(OriginBlockType::StyleRule, content, loc, option, parent);
    match obj.parse() {
      Ok(_) => {}
      Err(msg) => { return Err(msg); }
    }
    Ok(obj)
  }

  ///
  /// 创建变量
  ///
  pub fn create_var(content: String, loc: Loc, option: ParseOption) -> OriginBlock {
    OriginBlock::new(OriginBlockType::Var, content, loc, option, None)
  }

  fn parse(&mut self) -> Result<(), String> {
    match self.parse_comment() {
      Ok(mut blocks) => {
        self.block_node.append(&mut blocks);
      }
      Err(msg) => {
        return Err(msg);
      }
    }
    match self.parse_import() {
      Ok(mut blocks) => {
        self.block_node.append(&mut blocks);
      }
      Err(msg) => {
        return Err(msg);
      }
    }
    match self.parse_var() {
      Ok(mut blocks) => {
        self.block_node.append(&mut blocks);
      }
      Err(msg) => {
        return Err(msg);
      }
    }
    match self.parse_rule() {
      Ok(mut blocks) => {
        self.block_node.append(&mut blocks);
      }
      Err(msg) => {
        return Err(msg);
      }
    }
    Ok(())
  }
}