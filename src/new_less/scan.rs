use crate::extend::vec_str::VecCharExtend;
use std::cell::RefCell;
use std::rc::Rc;

///
/// 扫词的 前 | 中 | 后 字符
///
pub type CharWord<'a> = (Option<&'a char>, &'a char, Option<&'a char>);

///
/// 基础可变参数
///
#[derive(Debug)]
pub struct ScanArg {
  pub index: usize,
  pub temp: Rc<RefCell<Vec<char>>>,
  pub hasend: bool,
}

pub enum ScanResult {
  Arg(ScanArg),
  Skip,
}

///
/// 遍历
///
pub fn traversal(
  arg_start: Option<usize>,
  charlist: &[char],
  exec: &mut dyn FnMut(ScanArg, CharWord) -> Result<ScanResult, String>,
) -> Result<(String, usize), String> {
  let mut index = arg_start.unwrap_or(0);
  let temp = Rc::new(RefCell::new(vec![]));
  let mut hasend = false;

  while index < charlist.len() {
    let prevchar = if index == 0 {
      None
    } else {
      charlist.get(index - 1)
    };
    let char = charlist.get(index).unwrap();
    let nextchar = if index + 1 < charlist.len() {
      charlist.get(index + 1)
    } else {
      None
    };
    let arg = ScanArg {
      index,
      temp: temp.clone(),
      hasend,
    };
    let res: ScanResult = match exec(arg, (prevchar, char, nextchar)) {
      Err(msg) => {
        return Err(msg);
      }
      Ok(obj) => obj,
    };
    match res {
      ScanResult::Arg(arg) => {
        index = arg.index;
        hasend = arg.hasend;
      }
      ScanResult::Skip => {}
    }
    if hasend {
      break;
    }
    index += 1;
  }
  let final_str = temp.borrow().poly();
  Ok((final_str, index))
}
