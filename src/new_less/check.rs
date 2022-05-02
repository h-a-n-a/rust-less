use crate::new_less::context::ContextAsync;
use std::fmt::Debug;

pub trait Plugin: Sync + Send + Debug {}

struct A {
  filed: ContextAsync,
}

impl Plugin for ContextAsync {}
