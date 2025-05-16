use crate::{
  destination::Destination, executor::ExecutorTrait, pipeline::Pipeline, processor::Processors,
  source::Source,
};

pub struct Flow {
  pub executors: Vec<Box<dyn ExecutorTrait>>,
}

impl Flow {
  pub fn new() -> Self {
    Flow {
      executors: Vec::new(),
    }
  }

  pub fn add_pipeline<
    S: Source<Send = P::Recv> + 'static,
    P: Processors + 'static,
    D: Destination<Recv = P::Send> + 'static,
  >(
    &mut self,
    pipeline: Pipeline<S, P, D>,
  ) -> &mut Self {
    let executor = crate::executor::Executor::new(pipeline, crate::context::Context::new());
    self.executors.push(Box::new(executor));
    self
  }

  pub fn run(&mut self) {
    for executor in &mut self.executors {
      executor.execute();
    }
  }
}
