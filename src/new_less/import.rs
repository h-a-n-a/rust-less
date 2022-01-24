use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;
use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::token::lib::Token;
use crate::new_less::token::select::{TokenAllow, TokenCombina, TokenSelect};
use std::ops::Deref;

///
/// import 处理
///
#[derive(Debug, Clone)]
pub struct Import {
    pub origin_txt: String,
    charlist: Vec<String>,
}

impl Import {
    ///
    /// 初始化方法
    ///
    pub fn new(txt: String) -> Result<Import, String> {
        let mut obj = Import {
            origin_txt: txt.to_string().clone(),
            charlist: txt.trim().to_string().tocharlist(),
        };

        match obj.parse() {
            Ok(()) => Ok(obj),
            Err(msg) => Err(msg),
        }
    }

    ///
    /// 解析 字符串
    ///
    fn parse(&mut self) -> Result<(), String> {
        let charlist = &self.charlist;
        let mut temp: String = "".to_string();

        let length = charlist.len();

        if length < 7
            || (length == 7 && charlist[0..7].poly().as_str() != "@import")
            || (length > 7 && charlist[0..8].poly().as_str() != "@import")
        {
            return Err(format!("select_txt not match import"));
        }

        let mut word_vec = vec!["@import".to_string()];

        let mut index = 7;

        while index < charlist.len() {
            let char = charlist.get(index).unwrap().to_string();
        }

        Ok(())
    }
}
