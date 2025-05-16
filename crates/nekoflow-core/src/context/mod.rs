use std::{any::TypeId, collections::HashMap, ops::Deref, sync::Arc};

pub struct _Context {
  state: HashMap<TypeId, Box<dyn std::any::Any>>,
}

impl _Context {
  pub fn new() -> Self {
    _Context {
      state: HashMap::new(),
    }
  }
}

#[derive(Clone)]
pub struct Context(Arc<_Context>);

impl Context {
  pub fn new() -> Self {
    Context(Arc::new(_Context::new()))
  }
}

impl Deref for Context {
  type Target = _Context;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
