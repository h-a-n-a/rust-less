use regex::Regex;
use crate::extend::js_arg::StrOrRegex;
use crate::extend::string::StringExtend;
use crate::less::parser::chunker::chunker;

#[allow(dead_code)]
const CHARCODE_SPACE: u32 = 32;
#[allow(dead_code)]
const CHARCODE_TAB: u32 = 9;
#[allow(dead_code)]
const CHARCODE_LF: u32 = 10;
#[allow(dead_code)]
const CHARCODE_CR: u32 = 13;
#[allow(dead_code)]
const CHARCODE_PLUS: u32 = 43;
#[allow(dead_code)]
const CHARCODE_COMMA: u32 = 44;
#[allow(dead_code)]
const CHARCODE_FORWARD_SLASH: u32 = 47;
#[allow(dead_code)]
const CHARCODE_9: u32 = 57;


#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct ParserInputEndResult {
  isFinished: bool,
  furthest: usize,
  furthestPossibleErrorMessage: String,
  furthestReachedEnd: bool,
  furthestChar: String,
}

#[allow(dead_code)]
pub struct SaveStack {
  current: String,
  i: usize,
  j: usize,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct Comment {
  index: usize,
  isLineComment: bool,
  text: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
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
  #[allow(non_snake_case)]
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
  
  #[allow(non_snake_case)]
  fn restore(&mut self, possibleErrorMessage: String) {
    if self.i > self.furthest || (self.i == self.furthest && !possibleErrorMessage.is_empty() && self.furthestPossibleErrorMessage.is_empty()) {
      self.furthest = self.i;
      self.furthestPossibleErrorMessage = possibleErrorMessage;
    }
    let state = self.saveStack.last().unwrap();
    self.current = (*state).current.clone();
    self.currentPos = state.i;
    self.i = state.i;
    self.j = state.j;
    if !self.saveStack.is_empty() {
      self.saveStack.remove(self.saveStack.len() - 1);
    }
  }
  
  #[allow(non_snake_case)]
  fn forget(&mut self) {
    if !self.saveStack.is_empty() {
      self.saveStack.remove(self.saveStack.len() - 1);
    }
  }
  
  #[allow(non_snake_case)]
  fn isWhitespace(&mut self, offset: Option<usize>) -> bool {
    let pos = self.i + offset.unwrap_or(0);
    let code = self.input.charCodeAt(pos).unwrap();
    code == CHARCODE_SPACE || code == CHARCODE_CR || code == CHARCODE_TAB || code == CHARCODE_LF
  }
  
  #[allow(non_snake_case)]
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
  
  #[allow(non_snake_case)]
  fn _char(&mut self, tok: String) -> Option<String> {
    if self.input.charAt(self.i) != Some(tok.clone()) {
      return None;
    }
    self.skipWhitespace(1);
    Some(tok)
  }
  
  #[allow(non_snake_case)]
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
  
  #[allow(non_snake_case)]
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
    None
  }
  
  #[allow(non_snake_case)]
  fn _parseUntil(&mut self, tok: StrOrRegex) -> Option<String> {
    let mut quote: Vec<String>;
    let mut returnVal: Option<String> = None;
    let mut inComment = false;
    let mut blockDepth = 0;
    let mut blockStack: Vec<String> = vec![];
    let mut parseGroups: Vec<String> = vec![];
    let length = self.input.len();
    let startPos = self.i;
    let mut lastPos = self.i;
    let mut i = self.i;
    let mut loops = true;
    let testChar: Box<dyn Fn(String) -> bool>;
    
    match tok {
      StrOrRegex::Regex(regex) => {
        testChar = Box::new(move |char: String| { regex.is_match(char.as_str()) });
      }
      StrOrRegex::Str(txt) => {
        testChar = Box::new(move |char: String| { char == txt });
      }
    }
    
    loop {
      let mut nextChar = self.input.charAt(i).unwrap();
      if blockDepth == 0 && testChar(nextChar.clone()) {
        returnVal = Some(self.input.substr(lastPos as i32, Some((i - lastPos) as i32)));
        if returnVal.is_some() {
          parseGroups.push(returnVal.unwrap().clone());
        } else {
          parseGroups.push(" ".to_string());
        }
        returnVal = Some(parseGroups.join(""));
        self.skipWhitespace(i - startPos);
        loops = false;
      } else {
        if inComment {
          if nextChar == "*".to_string() && self.input.charAt(i + 1).unwrap_or("".to_string()) == "/".to_string() {
            i += 1;
            blockDepth -= 1;
            inComment = false;
          }
          i += 1;
          continue;
        }
        match nextChar.as_str() {
          r#"\\"# => {
            i += 1;
            nextChar = self.input.charAt(i).unwrap();
            parseGroups.push(
              self.input.substr(
                lastPos as i32,
                Some((i - lastPos + 1) as i32),
              )
            );
            lastPos = i + 1;
          }
          "/" => {
            if self.input.charAt(i + 1) == Some("*".to_string()) {
              i += 1;
              inComment = true;
              blockDepth += 1
            }
          }
          r#"\"# | r#"""# => {
            quote = self._quoted(Some(i)).unwrap_or(vec![]);
            if !quote.is_empty() {
              parseGroups.push(
                self.input.substr(
                  lastPos as i32,
                  Some((i - lastPos) as i32),
                )
              );
              parseGroups.push(quote.join(""));
              i += quote[1].len() - 1;
              lastPos = i + 1;
            } else {
              self.skipWhitespace(i - startPos);
              returnVal = Some(nextChar.clone());
              loops = false;
            }
          }
          r#"{"# => {
            blockStack.push("}".to_string());
            blockDepth += 1;
          }
          r#"("# => {
            blockStack.push(")".to_string());
            blockDepth += 1;
          }
          r#"["# => {
            blockStack.push("]".to_string());
            blockDepth += 1;
          }
          r#"}"# | r#")"# | r#"]"# => {
            let expected: Option<String> = match blockStack.last() {
              None => { None }
              Some(val) => { Some(val.clone().to_string()) }
            };
            blockStack.remove(blockStack.len() - 1);
            if expected == Some(nextChar.clone()) {
              blockDepth -= 1;
            } else {
              self.skipWhitespace(i - startPos);
              returnVal = expected;
              loops = false;
            }
          }
          _ => {}
        }
        i += 1;
        if i > length {
          loops = false;
        }
      }
      let _prevChar = nextChar.clone();
      if !loops {
        break;
      }
    }
    returnVal
  }
  
  #[allow(non_snake_case)]
  fn peekChar(&mut self, tok: String) -> bool {
    self.input.charAt(self.i) == Some(tok)
  }
  
  #[allow(non_snake_case)]
  fn currentChar(&mut self) -> String {
    self.input.charAt(self.i).unwrap_or("".to_string())
  }
  
  #[allow(non_snake_case)]
  fn prevChar(&mut self) -> String {
    self.input.charAt(self.i - 1).unwrap_or("".to_string())
  }
  
  #[allow(non_snake_case)]
  fn getInput(&mut self) -> String {
    self.input.clone()
  }
  
  #[allow(non_snake_case)]
  fn peekNotNumeric(&mut self) -> bool {
    let cc = self.input.charCodeAt(self.i).unwrap();
    (cc > CHARCODE_9 || cc < CHARCODE_PLUS) || cc == CHARCODE_FORWARD_SLASH || cc == CHARCODE_COMMA
  }
  
  #[allow(non_snake_case)]
  fn start(&mut self, str: String, chunkInput: bool) -> Result<(), String> {
    self.input = str.clone();
    self.i = 0;
    self.j = 0;
    self.furthest = 0;
    if chunkInput {
      match chunker(str.clone()) {
        Ok(value) => {
          self.chunks = value;
        }
        Err(msg) => {
          return Err(msg);
        }
      };
    } else {
      self.chunks = vec![str.clone()]
    }
    match self.chunks.get(0) {
      None => { return Err("parserInput.chunks is empty!..".to_string()); }
      Some(value) => {
        self.current = value.clone();
      }
    };
    self.skipWhitespace(0);
    Ok(())
  }
  
  #[allow(non_snake_case)]
  fn end(&mut self) -> ParserInputEndResult {
    let mut msg = "".to_string();
    let isFinished = self.i >= self.input.len();
    if self.i < self.furthest {
      msg = self.furthestPossibleErrorMessage.clone();
      self.i = self.furthest;
    }
    ParserInputEndResult {
      isFinished,
      furthest: self.i.clone(),
      furthestPossibleErrorMessage: msg,
      furthestReachedEnd: self.i >= self.input.len() - 1,
      furthestChar: self.input.charAt(self.i).unwrap_or("".to_string()),
    }
  }
  
  fn new() -> ParserInput {
    ParserInput {
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
    }
  }
}

#[allow(non_snake_case)]
pub fn parser_input() -> ParserInput {
  let parserInput = ParserInput::new();
  parserInput
}