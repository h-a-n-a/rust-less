use regex::Regex;
use crate::extend::string::StringExtend;

const CHARCODE_SPACE: u32 = 32;
const CHARCODE_TAB: u32 = 9;
const CHARCODE_LF: u32 = 10;
const CHARCODE_CR: u32 = 13;
const CHARCODE_PLUS: u32 = 43;
const CHARCODE_COMMA: u32 = 44;
const CHARCODE_FORWARD_SLASH: u32 = 47;
const CHARCODE_9: u32 = 57;

struct SaveStack {
  current: String,
  i: usize,
  j: usize,
}

pub struct Comment {
  index: usize,
  isLineComment: bool,
  text: String,
}

pub struct ParserInput {
  finished: bool,
  autoCommentAbsorb: bool,
  commentStore: Vec<Comment>,
  i: usize,
  //无意义计算引用
  input: String,
  j: usize,
  saveStack: Vec<SaveStack>,
  furthest: usize,
  furthestPossibleErrorMessage: String,
  chunks: Vec<String>,
  current: String,
  currentPos: usize,
  
}

impl ParserInput {
  fn skipWhitespace(&mut self, length: usize) -> bool {
    let mut parserInput = self;
    let oldi = parserInput.i;
    let oldj = parserInput.j;
    let curr = parserInput.i - parserInput.j;
    let endIndex = parserInput.i + parserInput.current.len() - curr;
    parserInput.i += length;
    let mem = parserInput.i;
    let inp = parserInput.input.clone();
    let mut c: u32;
    let mut nextChar: String;
    let mut comment: Comment;
    loop {
      if parserInput.i < endIndex {
        c = inp.charCodeAt(parserInput.i).unwrap();
        if parserInput.autoCommentAbsorb && c == CHARCODE_FORWARD_SLASH {
          nextChar = inp.charAt(parserInput.i + 1).unwrap();
          if nextChar == "/" {
            comment = Comment {
              index: parserInput.i,
              isLineComment: true,
              text: "".to_string(),
            };
            let mut nextNewLine = inp.indexOf("\n", Some(parserInput.i + 2));
            if nextNewLine < 0 {
              nextNewLine = endIndex as i32;
            }
            parserInput.i = nextNewLine as usize;
            comment.text = inp.substr(comment.index as i32, Some((parserInput.i - comment.index) as i32));
            parserInput.commentStore.push(comment);
            continue;
          } else if nextChar == "*" {
            let nextStarSlash = inp.indexOf("*/", Some(parserInput.i + 2));
            if nextStarSlash >= 0 {
              comment = Comment {
                index: parserInput.i,
                text: inp.substr(parserInput.i as i32, Some(nextStarSlash + 2 - parserInput.i as i32)),
                isLineComment: false,
              };
              parserInput.i += comment.text.len() - 1;
              parserInput.commentStore.push(comment);
              continue;
            }
          }
          break;
        }
        
        if c != CHARCODE_SPACE && c != CHARCODE_LF && c != CHARCODE_TAB && c != CHARCODE_CR {
          break;
        }
      } else {
        break;
      }
      parserInput.i += 1;
    }
    
    parserInput.current = parserInput.current.slice((length + parserInput.i - mem + curr) as i32);
    parserInput.currentPos = parserInput.i;
    
    if parserInput.current.is_empty() {
      if parserInput.j < parserInput.chunks.len() - 1 {
        parserInput.j += 1;
        parserInput.current = parserInput.chunks.get(parserInput.j).unwrap_or(&"".to_string()).to_string();
        parserInput.skipWhitespace(0);
        return true;
      }
      parserInput.finished = true;
    }
    
    oldi != parserInput.i || oldj != parserInput.j
  }
  
  fn save(&mut self) {
    self.currentPos = self.i;
    self.saveStack.push(SaveStack {
      current: self.current.clone(),
      i: self.i,
      j: self.j,
    });
  }
  
  fn restore(&mut self, possibleErrorMessage: String) {
    if self.i > self.furthest || (self.i == self.furthest && !possibleErrorMessage.is_empty() && self.furthestPossibleErrorMessage.is_empty()) {
      self.furthest = self.i;
      self.furthestPossibleErrorMessage = possibleErrorMessage;
    }
    let mut state = self.saveStack.last().unwrap();
    let rm_index = self.saveStack.len() - 1;
    self.current = (*state).current.clone();
    self.currentPos = state.i;
    self.i = state.i;
    self.j = state.j;
    if rm_index >= 0 {
      self.saveStack.remove(rm_index);
    }
  }
  
  fn forget(&mut self) {
    if !self.saveStack.is_empty() {
      self.saveStack.remove(self.saveStack.len() - 1);
    }
  }
  
  fn isWhitespace(&mut self, offset: Option<usize>) -> bool {
    let pos = self.i + offset.unwrap_or(0);
    let code = self.input.charCodeAt(pos).unwrap();
    code == CHARCODE_SPACE || code == CHARCODE_CR || code == CHARCODE_TAB || code == CHARCODE_LF
  }
  
  fn _re(&mut self, tok: Regex) -> Option<String> {
    if self.i > self.currentPos {
      self.current = self.current.slice((self.i - self.currentPos) as i32);
      self.currentPos = self.i;
    }
    let m = tok.captures(self.current.as_str());
    match m {
      None => {
        None
      }
      Some(capture) => {
        let value = capture.get(0).unwrap().as_str().to_string();
        self.skipWhitespace(value.len());
        Some(value)
      }
    }
  }
  
  fn _char(&mut self, tok: String) -> Option<String> {
    if self.input.charAt(self.i) != Some(tok.clone()) {
      return None;
    }
    self.skipWhitespace(1);
    Some(tok)
  }
  
  fn _str(&mut self, tok: String) -> Option<String> {
    let tokLength = tok.len();
    let mut i = 0;
    loop {
      if i < tokLength {
        if self.input.charAt(self.i + i).unwrap_or("".to_string()) != tok.charAt(i).unwrap_or("".to_string()) {
          return None;
        }
      } else {
        break;
      }
      i += 1;
    }
    self.skipWhitespace(tokLength);
    Some(tok)
  }
  
  fn _quoted(&mut self, loc: Option<usize>) -> Option<Vec<String>> {
    let pos = loc.unwrap_or(self.i);
    let startChar = self.input.charAt(pos).unwrap_or("".to_string());
    if startChar != "\'" && startChar != "\"" {
      return None;
    }
    let length = self.input.len();
    let currentPosition = pos;
    
    let mut i = 1;
    loop {
      if i + currentPosition < length {
        let nextChar = self.input.charAt(i + currentPosition).unwrap_or("".to_string());
        match nextChar.as_str() {
          "\\" => {
            i += 1;
            continue;
          }
          "\r" | "\n" => {
            break;
          }
          _ => {}
        }
        if startChar == nextChar {
          let str_ = self.input.substr(currentPosition as i32, Some((i + 1) as i32));
          if loc.unwrap_or(0) != 0 {
            self.skipWhitespace(i + 1);
            return Some(vec![str_]);
          }
          return Some(vec![startChar, str_]);
        }
      } else {
        break;
      }
      i += 1;
    };
    
    None;
  }
}

pub fn parser_input() -> ParserInput {
  let mut parserInput = ParserInput {
    finished: false,
    autoCommentAbsorb: true,
    commentStore: vec![],
    i: 0,
    //无意义计算引用
    input: "".to_string(),
    j: 0,
    saveStack: vec![],
    furthest: 0,
    furthestPossibleErrorMessage: "".to_string(),
    chunks: vec![],
    current: "".to_string(),
    currentPos: 0,
    
  };
  
  parserInput
}