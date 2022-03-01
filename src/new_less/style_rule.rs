use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;
use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::FileWeakRef;
use crate::new_less::ident::IdentType;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{HandleResult, NodeWeakRef, StyleNode, VarRuleNode};
use crate::new_less::option::ParseOption;
use crate::new_less::scan::{traversal, ScanArg, ScanResult};
use crate::new_less::token::lib::Token;
use crate::new_less::token::style_rule::TokenStyleRuleKeyAllow;
use crate::new_less::value::ValueNode;
use serde::Serialize;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct StyleRuleNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Option<Loc>,

  // 字符串 操作 序列
  #[serde(skip_serializing)]
  charlist: Vec<String>,

  // uuid 避免 查找时循环引用
  pub uuid: String,

  // 内部处理 地图
  #[serde(skip_serializing)]
  map: LocMap,

  // 文件信息
  #[serde(skip_serializing)]
  pub fileinfo: FileWeakRef,

  // 节点 父节点
  #[serde(skip_serializing)]
  pub parent: NodeWeakRef,

  // 上下文
  #[serde(skip_serializing)]
  pub context: ParseContext,

  // 对应的 key 值
  pub key: Option<String>,

  // 对应 值
  pub value: Option<ValueNode>,
}

impl Debug for StyleRuleNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ValueNode")
      .field("content", &self.content)
      .field("loc", &self.loc)
      .field("uuid", &self.uuid)
      .field("key", &self.key)
      .field("value", &self.value)
      .finish()
  }
}

impl StyleRuleNode {
  pub fn new(
    txt: String,
    loc: Option<Loc>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
    context: ParseContext,
  ) -> HandleResult<Self> {
    let map = if loc.is_none() {
      LocMap::new(txt.clone())
    } else {
      LocMap::merge(loc.as_ref().unwrap(), &txt).0
    };
    let mut obj = Self {
      content: txt.clone(),
      loc,
      charlist: txt.tocharlist(),
      uuid: Uuid::new_v4().to_string(),
      map,
      fileinfo,
      parent,
      context,
      key: None,
      value: None,
    };
    match obj.parse() {
      Ok(_) => HandleResult::Success(obj),
      Err(msg) => HandleResult::Fail(msg),
    }
  }

  ///
  /// 获取选项
  ///
  pub fn get_options(&self) -> ParseOption {
    self.context.borrow().option.clone()
  }

  ///
  /// 判断是否是 顶层 节点 下的变量
  ///
  pub fn is_top(&self) -> bool {
    self.parent.is_none()
  }

  ///
  /// 报错信息
  ///
  pub fn error_msg(&self, index: &usize) -> String {
    let error_loc = self.map.get(index).unwrap();
    let char = self.charlist.get(*index).unwrap().to_string();
    format!(
      "text {}, char {} is not allow, line is {} col is {}",
      &self.content, char, error_loc.line, error_loc.col
    )
  }

  ///
  /// 转化变量声明 key
  ///
  pub fn parse_var_ident(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;

    let res = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let ScanArg {
          mut temp,
          index,
          mut hasend,
        } = arg;
        let (_, char, next) = charword;
        if Token::is_token(&char) {
          if TokenStyleRuleKeyAllow::is(&char) {
            if char == TokenStyleRuleKeyAllow::Colon.tostr_value() {
              hasend = true;
            } else {
              temp += &char;
            }
          } else if Token::is_space_token(&char) {
            if Token::is_space_token(&next) {
              return Ok(ScanResult::Skip);
            } else if next == TokenStyleRuleKeyAllow::Colon.tostr_value() {
              temp += &char;
            } else {
              return Ok(ScanResult::Skip);
            }
          } else {
            return Err(self.error_msg(&index));
          }
        } else {
          temp += &char;
        }

        let new_arg = ScanArg {
          index,
          temp,
          hasend,
        };
        Ok(ScanResult::Arg(new_arg))
      }),
    )?;

    Ok(res)
  }

  ///
  /// 转化变量声明 value
  ///
  pub fn parse_var_value(&self, start: &usize) -> Result<(ValueNode, usize), String> {
    // 取分号前一位 最后一定是分号
    let end = self.charlist.len() - 1;
    let mut trim_start = *start;
    while trim_start < self.charlist.len() {
      if !Token::is_space_token(self.charlist.get(trim_start).unwrap()) {
        break;
      }
      trim_start += 1;
    }
    let content = self.charlist[trim_start..end].poly().trim().to_string();
    let node = ValueNode::new(
      content,
      self.map.get(start),
      self.parent.clone(),
      self.fileinfo.clone(),
    )?;
    Ok((node, self.charlist.len() - 1))
  }

  ///
  /// 转化校验
  ///
  fn parse(&mut self) -> Result<(), String> {
    let charlist = &self.charlist.clone();
    if charlist.is_empty() {
      return Err("var declare text is empty".to_string());
    }
    traversal(
      None,
      charlist,
      &mut (|arg, _| {
        let mut index = arg.index;
        if self.key.is_none() {
          let (key, jump) = self.parse_var_ident(&arg.index)?;
          index = jump;
          self.key = Some(key);
        } else if self.value.is_none() {
          let (value, jump) = self.parse_var_value(&arg.index)?;
          index = jump;
          self.value = Some(value);
        } else if self.value.is_some() && self.key.is_some() {
          return Err(self.error_msg(&index));
        }
        let new_arg = ScanArg {
          index,
          temp: arg.temp,
          hasend: false,
        };
        Ok(ScanResult::Arg(new_arg))
      }),
    )?;

    Ok(())
  }

  ///
  /// 代码生成
  ///
  pub fn code_gen(&self) -> Result<String, String> {
    let no_var_list = self.get_no_var_ident_list()?;
    let res = Self::group_calc_ident_value(no_var_list)?;
    let code_res = format!("{} : {}", self.key.as_ref().unwrap(), res);
    Ok(code_res)
  }

  ///
  /// 计算 提纯后 根据所有 词的 性质进行组合
  /// 用于 (运算)
  ///
  pub fn group_calc_ident_value(mut list: Vec<IdentType>) -> Result<String, String> {
    list.reverse();
    let mut nature_list: Vec<IdentType> = vec![];
    let mut calc_list: Vec<IdentType> = vec![];
    let mut index = 0;

    // 逆向查找第一个 非空格 的元素
    // 左值 重要
    let find_no_space_node_rev = |nlist: &Vec<IdentType>| {
      for item in nlist.iter().rev() {
        if !matches!(item, IdentType::Space) {
          return Some(item.clone());
        }
      }
      None
    };

    // 遍历 范式
    while index < list.len() {
      // 比对词性
      let now = list.get(index).unwrap().clone();
      match now {
        IdentType::Operator(op) => {
          if !calc_list.is_empty() {
            let last_calc_item = find_no_space_node_rev(&calc_list).unwrap();
            if matches!(last_calc_item, IdentType::Number(..)) {
              calc_list.push(IdentType::Operator(op));
            } else {
              // let json = serde_json::to_string_pretty(&calc_list).unwrap();
              // println!("{}", json);
              let json = serde_json::to_string_pretty(&list).unwrap();
              println!("{}", json);

              return Err(format!("operatar char is repeat {}", op));
            }
          } else {
            nature_list.push(IdentType::Word(op));
          }
        }
        IdentType::Number(..) => {
          if calc_list.is_empty() {
            calc_list.push(now);
          } else {
            let last_calc_item = find_no_space_node_rev(&calc_list).unwrap();
            if matches!(last_calc_item, IdentType::Operator(..))
              || matches!(last_calc_item, IdentType::Brackets(..))
            {
              calc_list.push(now);
            } else if matches!(last_calc_item, IdentType::Number(..)) {
              let calc_number = IdentType::calc_value(calc_list.clone())?;
              nature_list.push(calc_number);
              calc_list.clear();
              calc_list.push(now);
            }
          }
        }
        IdentType::Var(_) => {
          return Err("get_no_var_ident_list -> func has error!".to_string());
        }
        IdentType::Prop(_) => {
          return Err("$abc is not support".to_string());
        }
        IdentType::InsertVar(_) => {
          return Err("@{abc} is not support".to_string());
        }
        IdentType::StringConst(op)
        | IdentType::Word(op)
        | IdentType::Color(op)
        | IdentType::KeyWord(op) => {
          if !calc_list.is_empty() {
            let calc_number = IdentType::calc_value(calc_list.clone())?;
            nature_list.push(calc_number);
            calc_list.clear();
          }
          nature_list.push(IdentType::Word(op));
        }
        IdentType::Space => {
          if !calc_list.is_empty() {
            calc_list.push(now);
          } else {
            nature_list.push(now);
          }
        }
        IdentType::Escaping(_) => {
          return Err("(min-width: 768px) | ~'min-width: 768px'  is not support".to_string());
        }
        IdentType::Brackets(br) => {
          if !calc_list.is_empty() {
            calc_list.push(IdentType::Brackets(br));
          } else {
            nature_list.push(IdentType::Brackets(br));
          }
        }
      }
      index += 1;
    }
    if !calc_list.is_empty() {
      let calc_number = IdentType::calc_value(calc_list.clone())?;
      nature_list.push(calc_number);
      calc_list.clear();
    }

    let mut res: Vec<String> = vec![];
    for (index, item) in nature_list.iter().enumerate() {
      let last = if index > 0 {
        Some(nature_list.get(index - 1).unwrap().clone())
      } else {
        None
      };

      match item {
        IdentType::Number(value, unit) => {
          let add_char = "".to_string() + value + unit.clone().unwrap_or("".to_string()).as_str();
          if matches!(last, Some(IdentType::Word(..)))
            || matches!(last, Some(IdentType::Number(..)))
          {
            res.push(" ".to_string());
          }
          res.push(add_char);
        }
        IdentType::Word(char) => {
          if matches!(last, Some(IdentType::Word(..)))
            || matches!(last, Some(IdentType::Number(..)))
          {
            res.push(" ".to_string());
          }
          res.push(char.to_string());
        }
        IdentType::Space => {
          if !matches!(last, Some(IdentType::Space)) {
            res.push(" ".to_string());
          }
        }
        IdentType::Brackets(br) => {
          // todo fix single number situation
          res.push(br.to_string());
        }
        _ => {}
      }
    }

    Ok(res.join(""))
  }

  ///
  /// 代码转化 都 转化成 无变量 实参
  /// 用于 (变量计算)
  ///
  pub fn get_no_var_ident_list(&self) -> Result<Vec<IdentType>, String> {
    let mut list = self.value.as_ref().unwrap().word_ident_list.clone();
    if list.is_empty() {
      return Err(format!(
        "code_gen content {} is has error, value ident is empty!",
        self.content
      ));
    }
    // 把 表达式中 含有 var 声明的 全部进行 查找替换
    self.pure_list(&mut list)?;
    Ok(list)
  }

  ///
  /// 递归净化 所有表达式 的 var
  /// 用于 (变量计算)
  ///
  pub fn pure_list(&self, list: &mut Vec<IdentType>) -> Result<(), String> {
    let mut handle_vec: Vec<(usize, Vec<IdentType>)> = vec![];
    for (index, ident) in list.iter().enumerate() {
      if let IdentType::Var(ident_var) = ident {
        let var_node_value =
          self.get_var_by_key(ident_var, self.parent.clone(), self.fileinfo.clone())?;
        handle_vec.push((index, var_node_value.word_ident_list.clone()));
      }
    }
    // 把当前 所有的 变量 -> 代数 ident 插到 目前  ident_list vec 上
    for (index, ident_list) in handle_vec {
      list.remove(index);
      let mut setp = 0;
      ident_list.iter().rev().for_each(|x| {
        list.insert(index + setp, x.clone());
        setp += 1;
      });
    }
    let _json = serde_json::to_string_pretty(&list).unwrap();
    // 如果 当前 还有变量 则继续递归 演算
    if list.iter().any(|x| matches!(x, IdentType::Var(_))) {
      self.pure_list(list)?;
    };
    Ok(())
  }

  ///
  /// 查找变量
  /// 用于 (变量计算)
  ///
  pub fn get_var_by_key(
    &self,
    key: &str,
    rule_info: NodeWeakRef,
    file_info: FileWeakRef,
  ) -> Result<ValueNode, String> {
    if let Some(rule_ref) = rule_info {
      let rule = rule_ref.upgrade().unwrap();
      let nodelist = &rule.borrow().block_node;
      for item in nodelist {
        if let StyleNode::Var(VarRuleNode::Var(var)) = item.deref() {
          if var.key.as_ref().unwrap() == key {
            return Ok(var.value.as_ref().unwrap().clone());
          }
        }
      }
      return if rule.borrow().parent.is_some() {
        // 非顶层 向上递归
        self.get_var_by_key(key, rule.borrow().parent.clone(), None)
      } else {
        // 顶层 同层 引用递归 查看下半段代码
        self.get_var_by_key(key, None, self.fileinfo.clone())
      };
    }
    // 到达顶层后 取当前文件 的 顶层变量 或者 其他引用 文件的 顶层变量
    else if let Some(file_ref) = file_info {
      // 若没有则已经到达 顶层 则按照 顶层处理
      let fileinfo_ref = file_ref.upgrade().unwrap();
      let nodelist = &fileinfo_ref.borrow().block_node;
      for item in nodelist {
        if let StyleNode::Var(VarRuleNode::Var(var)) = item.deref() {
          if var.key.as_ref().unwrap() == key {
            return Ok(var.value.as_ref().unwrap().clone());
          }
        }
      }
      // 获取 其他 引用文件 顶层变量
      let top_level_other_vars = fileinfo_ref.borrow().collect_vars();
      for var in top_level_other_vars {
        if var.key.as_ref().unwrap() == key {
          return Ok(var.value.as_ref().unwrap().clone());
        }
      }
    };

    Err(format!("no var key {} has found", key))
  }
}
