use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

///
/// 内置结构体
///
pub struct HookData<T> {
  pub value: Option<T>,
}

///
/// 仿照 React-hooks 把String 放到 heap 上 来执行
///
pub fn create_hooks_str(init: Option<String>) -> (Weak<RefCell<HookData<String>>>, Box<dyn Fn(&str)>) {
  let content = Rc::new(RefCell::new(HookData { value: init }));
  let weak_ref = Rc::downgrade(&content);
  let change: Box<dyn Fn(&str)> = Box::new(move |txt: &str| {
    let value_now = content.deref().borrow().deref().value.clone();
    if value_now.is_none() {
      content.borrow_mut().deref_mut().value = Some(txt.to_string());
    } else {
      content.borrow_mut().deref_mut().value = Some(value_now.unwrap().clone() + txt);
    }
  });
  (weak_ref, change)
}