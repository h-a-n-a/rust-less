///
/// 扫词的 前 | 中 | 后 字符
///
pub type CharWord = (String, String, String);

///
/// 基础可变参数
///
#[derive(Clone, Debug, PartialEq)]
pub struct ScanArg {
  pub index: usize,
  pub temp: String,
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
  charlist: &Vec<String>,
  exec: &mut dyn FnMut(ScanArg, CharWord) -> Result<ScanResult, String>,
) -> Result<(String, usize), String> {
  let mut index = arg_start.unwrap_or(0);
  let mut temp: String = "".to_string();
  let mut hasend = false;
  
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
    let arg = ScanArg {
      index,
      temp: temp.clone(),
      hasend,
    };
    let res: ScanResult = match exec(arg, (prevchar, char, nextchar)) {
      Err(msg) => {
        return Err(msg);
      }
      Ok(obj) => {
        obj
      }
    };
    match res {
      ScanResult::Arg(arg) => {
        index = arg.index;
        temp = arg.temp;
        hasend = arg.hasend;
      }
      ScanResult::Skip => {
      
      }
    }
    if hasend {
      break;
    }
    index += 1;
  }
  Ok((temp, index))
}
