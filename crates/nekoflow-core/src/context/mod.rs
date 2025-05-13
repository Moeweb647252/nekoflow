use std::{any::TypeId, collections::HashMap, ops::Deref, sync::Arc};

struct _Context {
  state: HashMap<TypeId, Box<dyn std::any::Any>>,
}

#[derive(Clone)]
pub struct Context(Arc<_Context>);

impl Deref for Context {
  type Target = _Context;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
