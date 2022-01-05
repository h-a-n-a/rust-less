use std::collections::HashMap;
use std::ops::Deref;
use crate::extend::js::JsValue::JsObject;

#[derive(Clone)]
pub enum JsValue {
  JsStr(String),
  JsNum(i32),
  JsArray(Vec<JsValue>),
  JsObject(HashMap<String, JsValue>),
}

impl JsValue {
  fn num(&self) -> Option<i32> {
    let res = match self {
      JsValue::JsNum(num) => {
        Some(*num)
      }
      _ => { None }
    };
    res
  }
  
  fn new_obj() -> JsValue {
    let map = HashMap::new();
    JsValue::JsObject(map)
  }
  
  fn setkey(&mut self, key: String, val: JsValue) -> Result<(), String> {
    match self {
      JsObject(map) => {
        if map.get(key.as_str()).is_some() {
          map.get(key.as_str()).insert(&val);
        } else {
          map.insert(key, val).unwrap();
        }
        Ok(())
      }
      _ => {
        Err("set self is not JsObject".to_string())
      }
    }
  }
  
  fn getkey(&mut self, key: String) -> Result<Option<JsValue>, String> {
    match self {
      JsObject(map) => {
        if map.get(key.as_str()).is_some() {
          Ok(Some(map.get(key.as_str()).unwrap().deref().clone()))
        } else {
          Ok(None)
        }
      }
      _ => {
        Err("set self is not JsObject".to_string())
      }
    }
  }
}


