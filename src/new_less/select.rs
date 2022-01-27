use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;
use crate::extend::string::StringExtend;
use crate::new_less::token::lib::Token;
use crate::new_less::token::select::{TokenAllow, TokenCombina, TokenKeyWord, TokenSelect};
use serde::Serialize;
use std::ops::Deref;

///
/// 选择器范式
///
#[derive(Debug, PartialEq, Clone)]
pub enum SelectParadigm {
    // 选择器
    SelectWrap(String),

    // 选择链接器
    CominaWrap(String),

    // 其他token
    OtherWrap(String),

    // * 通配符号
    NormalWrap(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct Selector {
    pub origin_txt: String,
    pub single_select_txt: Vec<String>,
    charlist: Vec<String>,
}

impl Selector {
    ///
    /// 初始化方法
    ///
    pub fn new(txt: String) -> Result<Self, String> {
        let mut obj = Selector {
            origin_txt: txt.trim().to_string(),
            single_select_txt: vec![],
            charlist: txt.tocharlist(),
        };
        match obj.parse() {
            Ok(()) => Ok(obj),
            Err(msg) => Err(msg),
        }
    }

    pub fn value(&self) -> String {
        self.origin_txt.clone()
    }

    ///
    /// 合并范式内容
    ///
    pub fn join(paradigm: Vec<SelectParadigm>) -> String {
        let mut base = "".to_string();
        for word_paradigm in paradigm {
            match word_paradigm {
                SelectParadigm::SelectWrap(cc)
                | SelectParadigm::CominaWrap(cc)
                | SelectParadigm::OtherWrap(cc)
                | SelectParadigm::NormalWrap(cc) => {
                    base += &cc;
                }
            }
        }
        base
    }

    ///
    /// 打印错误信息
    ///
    fn errormsg(&mut self, index: &usize) -> Result<(), String> {
        let char = self.charlist.get(*index).unwrap().clone();
        Err(format!(
            "select text {}, char {} is not allow,index is {}",
            self.origin_txt, char, index
        ))
    }

    ///
    /// 判断相邻非空格字符串
    /// 当前索引位置 -> index
    /// 禁用单词 -> forbidword
    /// 默认查找方向 -> true | None 向后
    ///
    fn check_adjacent_token(
        &mut self,
        forbidword: Vec<&str>,
        index: &usize,
        forwad: Option<bool>,
    ) -> Result<(), String> {
        let back = forwad.unwrap_or(true);
        let mut find_num = *index;
        let to_move = |index: &mut usize| {
            let start = 0;
            let end = self.charlist.len() - 1;
            if back {
                if *index < end {
                    *index += 1;
                } else {
                    *index = end;
                }
            }
            if !back {
                if *index > start {
                    *index -= 1;
                } else {
                    *index = start;
                }
            }
        };
        let mut char;
        loop {
            to_move(&mut find_num);
            char = self.charlist.get(find_num).unwrap().deref();
            if char != TokenCombina::Space.tostr_value() {
                break;
            }
        }
        if Token::is_token(char) {
            // 验证 连接词 不能固定想连
            let res = forbidword.into_iter().find(|x| x == &char);
            match res {
                None => {}
                Some(_err_char) => {
                    return self.errormsg(&find_num);
                }
            }
        }
        Ok(())
    }

    ///
    /// 单独转化 attr 属性判断
    ///
    fn parse_attr(&mut self, start: &usize) -> Result<(SelectParadigm, usize), String> {
        let charlist = &self.charlist;
        let mut index = *start + 1;
        let mut temp: String = "[".to_string();
        // 是否完结
        let mut hasend = false;
        // 是否有等号
        let mut hasequal = false;
        // 是否有引号
        let mut has_quota = false;

        while index < charlist.len() {
            let prevchar = if index == 0 {
                "".to_string()
            } else {
                charlist.get(index - 1).unwrap().to_string()
            };
            let char = charlist.get(index).unwrap().to_string();
            let nextchar = if index == charlist.len() - 1 {
                "".to_string()
            } else {
                charlist.get(index + 1).unwrap().to_string()
            };
            let token_allow = vec!["^", "$", "*", "|"];
            // 如果重复遇到引号 则关闭 引号作用域
            if has_quota && (&char == r#"""# || &char == r#"'"#) {
                temp += &char;
                has_quota = false;
                index += 1;
                continue;
            }
            // 如果 引号关闭 且是 标点符号则执行检查
            if Token::is_token(&char) && !has_quota {
                // 遇到 "]" 则跳出循环 当前索引即是 "]" 的位置
                if &char == "]" {
                    hasend = true;
                    temp += &char;
                    break;
                }
                // 遇到 = 需要判断后一个词 只能跟 引号
                if &char == "=" {
                    // 不能有重复的等号出现
                    if !hasequal && (&nextchar == r#"""# || &nextchar == r#"'"#) {
                        // 且不能 是 [= 这种组合
                        if temp.len() > 1 {
                            hasequal = true;
                            temp += &char;
                            index += 1;
                            continue;
                        } else {
                            return Err(self.errormsg(&index).err().unwrap());
                        }
                    } else {
                        return Err(self.errormsg(&index).err().unwrap());
                    }
                }
                // 直接出现引号 没有出现等号 直接报错
                if &char == r#"""# || &char == r#"'"# {
                    if !hasequal {
                        return Err(self.errormsg(&index).err().unwrap());
                    } else {
                        // 前一个 符号必须是等号 这里重复判断可以优化!
                        if &prevchar == "=" {
                            has_quota = true;
                            temp += &char;
                            index += 1;
                            continue;
                        } else {
                            return Err(self.errormsg(&index).err().unwrap());
                        }
                    }
                }
                // 如果是其他符号 或者没有匹配的情况 则进行下述匹配
                if nextchar == "=" && token_allow.contains(&&*char) {
                    temp += &char
                } else {
                    return Err(self.errormsg(&index).err().unwrap());
                }
            } else {
                temp += &char
            }
            index += 1;
        }
        if !hasend {
            return Err(format!("select text {}, not found ']'", self.origin_txt));
        }
        let obj = SelectParadigm::SelectWrap(temp);
        Ok((obj, index))
    }

    ///
    /// 转小括号
    ///
    fn parse_brackets(&mut self, start: &usize) -> Result<(SelectParadigm, usize), String> {
        let charlist = &self.charlist;
        let mut index = *start + 1;
        let mut temp: String = "(".to_string();
        let mut hasend = false;

        while index < charlist.len() {
            let char = charlist.get(index).unwrap().to_string();
            if Token::is_token(&char) {
                if &char == "@" {
                    return Err(self.errormsg(&index).err().unwrap());
                } else {
                    temp += &char;
                    if char == TokenSelect::RightBrackets.tostr_value() {
                        hasend = true;
                        break;
                    }
                }
            } else {
                temp += &char
            }
            index += 1;
        }
        if !hasend {
            return Err(format!("select text {}, not found ')'", self.origin_txt));
        }
        let obj = SelectParadigm::SelectWrap(temp);
        Ok((obj, index))
    }

    ///
    /// 解析 字符串
    /// 验证有效性
    /// 根据 逗号 划分规则
    ///
    fn parse(&mut self) -> Result<(), String> {
        let charlist = self.charlist.clone();
        let mut index = 0;
        let mut temp: String = "".to_string();
        let mut paradigm_vec: Vec<SelectParadigm> = vec![];
        let mut has_ref_token = false;

        // 循环解析
        while index < charlist.len() {
            let prevchar = if index == 0 {
                "".to_string()
            } else {
                charlist.get(index - 1).unwrap().to_string()
            };
            let char = charlist.get(index).unwrap().to_string();
            let nextchar = if index == charlist.len() - 1 {
                "".to_string()
            } else {
                charlist.get(index + 1).unwrap().to_string()
            };

            // 跳过空格
            if Token::is_space_token(&char) && Token::is_space_token(&nextchar) {
                index += 1;
                continue;
            }
            // 有任务则继续填词
            if !Token::is_token(&char) {
                temp += &char.clone();
                if index + 1 != charlist.len() {
                    index += 1;
                    continue;
                }
            }

            if index == 0 {
                if Token::is_token(&char) {
                    if charlist.len() == 1 && char != TokenSelect::WildCard.tostr_value() {
                        return self.errormsg(&index);
                    }
                    // 第一个词 是符号
                    if TokenKeyWord::is(&char) {
                        // 第一个词 是 &
                        paradigm_vec.push(SelectParadigm::OtherWrap("$(&)".to_string()));
                        has_ref_token = true;
                    } else if TokenSelect::is(&char) {
                        // 第一个词 是 选择符号
                        match TokenSelect::try_from(char.clone().as_str()).unwrap() {
                            TokenSelect::ClassToken | TokenSelect::IdToken => {
                                temp += &char.clone();
                                // 起始符 后续不能接 任意 词根符 类似 "#>" ".*"
                                if Token::is_token(&nextchar) && !TokenAllow::is(&nextchar) {
                                    return self.errormsg(&(index + 1));
                                }
                            }
                            TokenSelect::Colon => {
                                temp += &char.clone();
                                if nextchar != TokenSelect::Colon.tostr_value()
                                    && Token::is_token(&nextchar)
                                {
                                    return self.errormsg(&(index + 1));
                                }
                            }
                            TokenSelect::AttrBegin => {
                                let (paradigm, jumpindex) = match self.parse_attr(&index) {
                                    Ok(res) => res,
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                };
                                paradigm_vec.push(paradigm);
                                index = jumpindex + 1;
                                continue;
                            }
                            TokenSelect::AttrEnd => {
                                return self.errormsg(&index);
                            }
                            TokenSelect::LeftBrackets => {
                                let (paradigm, jumpindex) = match self.parse_brackets(&index) {
                                    Ok(res) => res,
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                };
                                paradigm_vec.push(paradigm);
                                index = jumpindex + 1;
                                continue;
                            }
                            TokenSelect::RightBrackets => {
                                return self.errormsg(&index);
                            }
                            TokenSelect::WildCard => {
                                paradigm_vec.push(SelectParadigm::NormalWrap("*".to_string()));
                            }
                        }
                    } else if TokenCombina::is(&char) {
                        // 第一个词 是 链接符号 不考虑空格
                        match TokenCombina::try_from(char.clone().as_str()).unwrap() {
                            TokenCombina::Comma => {
                                return self.errormsg(&index);
                            }
                            TokenCombina::ExtendChar => {
                                paradigm_vec.push(SelectParadigm::CominaWrap(
                                    TokenCombina::ExtendChar.tostr_value(),
                                ));
                                if !Token::is_space_token(&nextchar) {
                                    paradigm_vec.push(SelectParadigm::CominaWrap(
                                        TokenCombina::Space.tostr_value(),
                                    ));
                                }
                                match self.check_adjacent_token(
                                    vec![
                                        "\n", "\r", "]", "&", "~", "+", "|", "~", ">", "'", r#"""#,
                                    ],
                                    &index,
                                    None,
                                ) {
                                    Ok(_) => {}
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                }
                            }
                            TokenCombina::ColumnChar => {
                                index += 1;
                                paradigm_vec.push(SelectParadigm::CominaWrap("||".to_string()));
                            }
                            TokenCombina::BrotherNextChar => {
                                paradigm_vec.push(SelectParadigm::CominaWrap(
                                    TokenCombina::BrotherNextChar.tostr_value(),
                                ));
                                // 补空格
                                if !Token::is_space_token(&nextchar) {
                                    paradigm_vec.push(SelectParadigm::CominaWrap(
                                        TokenCombina::Space.tostr_value(),
                                    ));
                                }
                                match self.check_adjacent_token(
                                    vec![
                                        "\n", "\r", "]", "&", "~", "+", "|", "~", ">", "'", r#"""#,
                                    ],
                                    &index,
                                    None,
                                ) {
                                    Ok(_) => {}
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                }
                            }
                            TokenCombina::BrotherMatchChar => {
                                paradigm_vec.push(SelectParadigm::CominaWrap(
                                    TokenCombina::BrotherMatchChar.tostr_value(),
                                ));
                                // 补空格
                                if !Token::is_space_token(&nextchar) {
                                    paradigm_vec.push(SelectParadigm::CominaWrap(
                                        TokenCombina::Space.tostr_value(),
                                    ));
                                }
                                match self.check_adjacent_token(
                                    vec![
                                        "\n", "\r", "]", "&", "~", "+", "|", "~", ">", "'", r#"""#,
                                    ],
                                    &index,
                                    None,
                                ) {
                                    Ok(_) => {}
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else if TokenAllow::is(&char) {
                        // 安全词 可以考虑按照 普通字符一样处理
                        temp += &char.clone();
                    } else {
                        // 非安全词 直接报错 排除了 括号 和 中括号 中 被引号处理的情况
                        return self.errormsg(&index);
                    }
                } else {
                    // 第一个词 非符号
                    temp += &char.clone();
                }
            } else if index == charlist.len() - 1 {
                // 结尾处理
                if Token::is_token(&char) {
                    // 处理字符
                    if TokenKeyWord::is(&char) {
                        // 第一个词 是 &
                        if !has_ref_token {
                            if !Token::is_space_token(&prevchar) {
                                paradigm_vec.push(SelectParadigm::CominaWrap(
                                    TokenCombina::Space.tostr_value(),
                                ));
                            }
                            paradigm_vec.push(SelectParadigm::OtherWrap("$(&)".to_string()));
                        }
                    } else if TokenSelect::is(&char) && char != TokenSelect::WildCard.tostr_value()
                    {
                        return self.errormsg(&index);
                    } else if TokenCombina::is(&char) {
                        match TokenCombina::try_from(char.as_str()).unwrap() {
                            TokenCombina::ColumnChar => {
                                return self.errormsg(&index);
                            }
                            _ => {
                                // 自动忽略字符
                                if !temp.is_empty() {
                                    paradigm_vec.push(SelectParadigm::SelectWrap(temp.clone()));
                                    temp = "".to_string();
                                }
                            }
                        }
                    } else if TokenAllow::is(&char) {
                        if char != TokenAllow::LeftSlant.tostr_value() {
                            paradigm_vec.push(SelectParadigm::OtherWrap(char.clone()));
                        } else {
                            return self.errormsg(&index);
                        }
                    } else {
                        return self.errormsg(&index);
                    }
                } else {
                    // 处理非字符
                    if !temp.is_empty() {
                        paradigm_vec.push(SelectParadigm::SelectWrap(temp.clone()));
                        temp = "".to_string();
                    }
                }
                if !paradigm_vec.is_empty() {
                    let single_select_txt = Self::join(paradigm_vec.clone());
                    self.single_select_txt.push(single_select_txt);
                    paradigm_vec = vec![];
                }
            } else {
                // 过程处理
                if Token::is_token(&char) {
                    if !temp.is_empty() {
                        paradigm_vec.push(SelectParadigm::SelectWrap(temp.clone()));
                        temp = "".to_string();
                    }
                    if TokenKeyWord::is(&char) {
                        if !has_ref_token {
                            if !Token::is_space_token(&prevchar) {
                                paradigm_vec.push(SelectParadigm::CominaWrap(
                                    TokenCombina::Space.tostr_value(),
                                ));
                            }
                            paradigm_vec.push(SelectParadigm::OtherWrap("$(&)".to_string()));
                            has_ref_token = true;
                        }
                    } else if TokenSelect::is(&char) {
                        // 词 是 选择符号
                        match TokenSelect::try_from(char.clone().as_str()).unwrap() {
                            TokenSelect::ClassToken | TokenSelect::IdToken => {
                                temp += &char.clone();
                                // 起始符 后续不能接 任意 词根符 类似 "#>" ".*"
                                if Token::is_token(&nextchar) && !TokenAllow::is(&nextchar) {
                                    return self.errormsg(&(index + 1));
                                }
                            }
                            TokenSelect::Colon => {
                                temp += &char.clone();
                                if nextchar != TokenSelect::Colon.tostr_value()
                                    && Token::is_token(&nextchar)
                                {
                                    return self.errormsg(&(index + 1));
                                }
                            }
                            TokenSelect::AttrBegin => {
                                let (paradigm, jumpindex) = match self.parse_attr(&index) {
                                    Ok(res) => res,
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                };
                                paradigm_vec.push(paradigm);
                                index = jumpindex + 1;
                                continue;
                            }
                            TokenSelect::AttrEnd => {
                                return self.errormsg(&index);
                            }
                            TokenSelect::LeftBrackets => {
                                let (paradigm, jumpindex) = match self.parse_brackets(&index) {
                                    Ok(res) => res,
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                };
                                paradigm_vec.push(paradigm);
                                index = jumpindex + 1;
                                continue;
                            }
                            TokenSelect::RightBrackets => {
                                return self.errormsg(&index);
                            }
                            TokenSelect::WildCard => {
                                paradigm_vec.push(SelectParadigm::NormalWrap("*".to_string()));
                            }
                        }
                    } else if TokenCombina::is(&char) {
                        match TokenCombina::try_from(char.clone().as_str()).unwrap() {
                            TokenCombina::Comma => {
                                let single_select_txt = Self::join(paradigm_vec.clone());
                                self.single_select_txt.push(single_select_txt);
                                has_ref_token = false;
                                paradigm_vec = vec![];
                            }
                            TokenCombina::Space
                            | TokenCombina::NewLineOs
                            | TokenCombina::NewLineWindos => {
                                if !Token::is_space_token(&prevchar) {
                                    paradigm_vec.push(SelectParadigm::CominaWrap(
                                        TokenCombina::Space.tostr_value(),
                                    ));
                                }
                                let space =
                                    SelectParadigm::CominaWrap(TokenCombina::Space.tostr_value());
                                if paradigm_vec.last().unwrap() != &space {
                                    paradigm_vec.push(space);
                                }
                            }
                            TokenCombina::ExtendChar => {
                                if !Token::is_space_token(&nextchar) {
                                    paradigm_vec.push(SelectParadigm::CominaWrap(
                                        TokenCombina::Space.tostr_value(),
                                    ));
                                }
                                paradigm_vec.push(SelectParadigm::CominaWrap(
                                    TokenCombina::ExtendChar.tostr_value(),
                                ));
                                if !Token::is_space_token(&nextchar) {
                                    paradigm_vec.push(SelectParadigm::CominaWrap(
                                        TokenCombina::Space.tostr_value(),
                                    ));
                                }
                                match self.check_adjacent_token(
                                    vec![
                                        "\n", "\r", "]", "&", "~", "+", "|", "~", ">", "'", r#"""#,
                                    ],
                                    &index,
                                    None,
                                ) {
                                    Ok(_) => {}
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                }
                            }
                            TokenCombina::ColumnChar => {}
                            TokenCombina::BrotherNextChar => {
                                if !Token::is_space_token(&prevchar) {
                                    paradigm_vec.push(SelectParadigm::CominaWrap(
                                        TokenCombina::Space.tostr_value(),
                                    ));
                                }
                                paradigm_vec.push(SelectParadigm::CominaWrap(
                                    TokenCombina::BrotherNextChar.tostr_value(),
                                ));
                                if !Token::is_space_token(&nextchar) {
                                    paradigm_vec.push(SelectParadigm::CominaWrap(
                                        TokenCombina::Space.tostr_value(),
                                    ));
                                }
                                match self.check_adjacent_token(
                                    vec![
                                        "\n", "\r", "]", "&", "~", "+", "|", "~", ">", "'", r#"""#,
                                    ],
                                    &index,
                                    None,
                                ) {
                                    Ok(_) => {}
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                }
                            }
                            TokenCombina::BrotherMatchChar => {
                                if !Token::is_space_token(&prevchar) {
                                    paradigm_vec.push(SelectParadigm::CominaWrap(
                                        TokenCombina::Space.tostr_value(),
                                    ));
                                }
                                paradigm_vec.push(SelectParadigm::CominaWrap(
                                    TokenCombina::BrotherMatchChar.tostr_value(),
                                ));
                                if !Token::is_space_token(&nextchar) {
                                    paradigm_vec.push(SelectParadigm::CominaWrap(
                                        TokenCombina::Space.tostr_value(),
                                    ));
                                }
                                match self.check_adjacent_token(
                                    vec![
                                        "\n", "\r", "]", "&", "~", "+", "|", "~", ">", "'", r#"""#,
                                    ],
                                    &index,
                                    None,
                                ) {
                                    Ok(_) => {}
                                    Err(msg) => {
                                        return Err(msg);
                                    }
                                }
                            }
                        }
                    } else {
                        // 其他非关键词根 过程处理
                        if !TokenAllow::is(&char) {
                            // 非安全词 直接报错 排除了 括号 和 中括号 中 被引号处理的情况
                            return self.errormsg(&index);
                        } else {
                            // 安全词 可以考虑按照 普通字符一样处理
                            temp += &char.clone();
                        }
                    }
                }
            }
            index += 1;
        }
        Ok(())
    }
}
