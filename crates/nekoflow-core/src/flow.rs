use crate::executor::ExecutorTrait;

pub struct Flow {
  pub executors: Vec<Box<dyn ExecutorTrait>>,
}
