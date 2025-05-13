use crate::pipeline::Pipeline;

pub struct Executor {}

pub trait ExecutorTrait {
  fn execute(&self);
}
