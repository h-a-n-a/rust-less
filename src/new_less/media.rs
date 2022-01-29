use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;
use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{HandleResult, NodeWeakRef};
use crate::new_less::scan::{traversal, ScanArg, ScanResult};
use crate::new_less::token::lib::Token;
use crate::new_less::token::media::{
  TokenMediaFeature, TokenMediaLogic, TokenMediaType, TokenMeidaAllow,
};
use serde::Serialize;

///
/// 媒体查询
///
#[derive(Debug, Clone, Serialize)]
pub struct MediaQuery {
  pub origin_txt: String,

  pub loc: Option<Loc>,

  #[serde(skip_serializing)]
  map: LocMap,

  #[serde(skip_serializing)]
  charlist: Vec<String>,

  #[serde(skip_serializing)]
  pub parent: NodeWeakRef,
}

impl MediaQuery {
  ///
  /// 初始化方法
  ///
  pub fn new(txt: String, loc: Option<Loc>, map: Option<LocMap>) -> HandleResult<Self> {
    let obj = Self {
      origin_txt: txt.clone(),
      loc,
      map: map.unwrap_or_else(|| {
        LocMap::new(txt.clone())
      }),
      charlist: txt.trim().to_string().tocharlist(),
      parent: None,
    };
    match obj.parse() {
      Ok(_) => HandleResult::Success(obj),
      Err(msg) => {
        if &msg == "select_txt not match media query" {
          HandleResult::Swtich
        } else {
          HandleResult::Fail(msg)
        }
      }
    }
  }

  ///
  /// 打印错误信息
  ///
  pub fn errormsg(&self, index: &usize) -> Result<(), String> {
    let char = self.charlist.get(*index).unwrap().clone();
    let error_loc = self.map.get(index).unwrap();
    Err(format!(
      "select text {}, char {} is not allow,line is {} col is {}",
      self.origin_txt, char, error_loc.line, error_loc.col
    ))
  }

  pub fn value(&self) -> String {
    self.origin_txt.clone()
  }

  ///
  /// 子转化 媒体功能 转化 key
  ///
  pub fn parse_media_feature_key(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    match traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let mut hasend = arg.hasend;
        let mut temp = arg.temp;
        let index = arg.index;
        let (_, char, next) = charword;
        if Token::is_token(&char) {
          if char == TokenMeidaAllow::Colon.tostr_value() {
            if TokenMediaFeature::is(temp.trim()) {
              // 加冒号之前 先判断是否是有效 key
              hasend = true;
            } else {
              return Err(self.errormsg(&index).err().unwrap());
            }
          } else if Token::is_space_token(&char) {
            if Token::is_space_token(&next) {
              return Ok(ScanResult::Skip);
            } else {
              temp += &char;
            }
          } else if &char == "-" {
            temp += "-";
          } else {
            return Err(self.errormsg(&index).err().unwrap());
          }
        } else {
          temp += &char;
        }
        Ok(ScanResult::Arg(ScanArg {
          temp,
          index,
          hasend,
        }))
      }),
    ) {
      Ok(res) => Ok(res),
      Err(msg) => Err(msg),
    }
  }

  ///
  /// 子转化 媒体功能 转化 value
  ///
  pub fn parse_media_value(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    match traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let mut hasend = arg.hasend;
        let mut temp = arg.temp;
        let index = arg.index;
        let (_, char, next) = charword;
        if Token::is_token(&char) {
          if char == TokenMeidaAllow::RightBrackets.tostr_value() {
            hasend = true;
          } else if Token::is_space_token(&char) {
            if Token::is_space_token(&next) {
              return Ok(ScanResult::Skip);
            } else {
              temp += &char;
            }
          } else if &char == "-" {
            if temp.trim().is_empty() {
              temp += "-";
            } else {
              return Err(self.errormsg(&index).err().unwrap());
            }
          } else {
            return Err(self.errormsg(&index).err().unwrap());
          }
        } else {
          temp += &char;
        }
        Ok(ScanResult::Arg(ScanArg {
          temp,
          index,
          hasend,
        }))
      }),
    ) {
      Ok(res) => Ok(res),
      Err(msg) => Err(msg),
    }
  }

  ///
  /// 子转化 媒体功能
  ///
  pub fn parse_media_feature(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let mut index = *start + 1;
    let mut word_vec: Vec<String> = vec!["(".to_string()];

    // 分析key
    let (key, jump) = match self.parse_media_feature_key(&index.clone()) {
      Ok(res) => res,
      Err(msg) => {
        return Err(msg);
      }
    };
    word_vec.push(key);
    word_vec.push(":".to_string());
    index = jump + 1;

    // 分析value
    let (value, jump) = match self.parse_media_value(&index.clone()) {
      Ok(res) => res,
      Err(msg) => {
        return Err(msg);
      }
    };
    word_vec.push(value);
    word_vec.push(")".to_string());
    index = jump + 1;

    if index < charlist.len() {
      return Err(self.errormsg(&index).err().unwrap());
    }

    Ok((word_vec.poly(), index))
  }

  ///
  /// 转化 逻辑词
  ///
  pub fn parse_media_logicword(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let (word, jump) = match traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let mut hasend = arg.hasend;
        let mut temp = arg.temp;
        let index = arg.index;
        let (_, char, _) = charword;
        if Token::is_token(&char) {
          if Token::is_space_token(&char) {
            hasend = true;
          } else {
            return Err(self.errormsg(&index).err().unwrap());
          }
        } else {
          temp += &char;
        }
        Ok(ScanResult::Arg(ScanArg {
          temp,
          index,
          hasend,
        }))
      }),
    ) {
      Ok(res) => res,
      Err(msg) => {
        return Err(msg);
      }
    };

    if TokenMediaLogic::is(&word) || TokenMediaType::is(&word) {
      Ok((word, jump))
    } else {
      Err(self.errormsg(&jump).err().unwrap())
    }
  }

  pub fn parse(&self) -> Result<(), String> {
    let charlist = &self.charlist;
    if charlist.len() < 6
      || (charlist.len() == 6 && charlist[0..6].poly().as_str() != "@media")
      || (charlist.len() > 6 && charlist[0..7].poly().as_str() != "@media ")
    {
      return Err("select_txt not match media query".to_string());
    }
    let mut word_vec = vec!["@media".to_string()];
    let index = 6;

    match traversal(
      Some(index),
      charlist,
      &mut (|arg, charword| {
        let temp = arg.temp;
        let mut index = arg.index;
        let (_, char, next) = charword;
        return if Token::is_token(&char) {
          if Token::is_space_token(&char) {
            if !Token::is_space_token(&next) {
              word_vec.push(" ".to_string());
              Ok(ScanResult::Skip)
            } else {
              Ok(ScanResult::Skip)
            }
          } else if TokenMeidaAllow::is(&char) {
            match TokenMeidaAllow::try_from(char.as_str()).unwrap() {
              TokenMeidaAllow::LeftBrackets => match self.parse_media_feature(&index) {
                Ok((word, jump)) => {
                  word_vec.push(word);
                  index = jump;
                  return Ok(ScanResult::Arg(ScanArg {
                    index,
                    temp,
                    hasend: false,
                  }));
                }
                Err(msg) => Err(msg),
              },
              _ => Err(self.errormsg(&index).err().unwrap()),
            }
          } else {
            Err(self.errormsg(&index).err().unwrap())
          }
        } else {
          let (word, jump) = match self.parse_media_logicword(&index) {
            Ok(res) => res,
            Err(msg) => {
              return Err(msg);
            }
          };
          index = jump;
          word_vec.push(word);
          word_vec.push(" ".to_string());
          Ok(ScanResult::Arg(ScanArg {
            index,
            temp,
            hasend: false,
          }))
        };
      }),
    ) {
      Ok(res) => res,
      Err(msg) => {
        return Err(msg);
      }
    };
    // println!("{:#?}", word_vec);
    Ok(())
  }
}
