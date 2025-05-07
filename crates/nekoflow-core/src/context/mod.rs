use std::{any::TypeId, collections::HashMap};

pub struct Context {
  state: HashMap<TypeId, Box<dyn std::any::Any>>,
}
