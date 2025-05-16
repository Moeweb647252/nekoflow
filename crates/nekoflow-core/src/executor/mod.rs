use crate::context::Context;
use crate::pipeline::Pipeline;
use crate::{destination::Destination, processor::Processors, source::Source};

pub struct Executor<S: Source<Send = P::Recv>, P: Processors, D: Destination<Recv = P::Send>> {
  pipeline: Pipeline<S, P, D>,
  context: Context,
}

impl<S: Source<Send = P::Recv>, P: Processors, D: Destination<Recv = P::Send>> Executor<S, P, D> {
  pub fn new(pipeline: Pipeline<S, P, D>, context: Context) -> Self {
    Executor { pipeline, context }
  }
}

pub trait ExecutorTrait {
  fn execute(&mut self);
}

impl<S: Source<Send = P::Recv>, P: Processors, D: Destination<Recv = P::Send>> ExecutorTrait
  for Executor<S, P, D>
{
  fn execute(&mut self) {
    // Execute the pipeline
    let source = &mut self.pipeline.source;
    let destination = &mut self.pipeline.destination;
    let processors = &mut self.pipeline.processors;

    tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .unwrap()
      .block_on(async {
        let data = source.get(self.context.clone()).await.unwrap();
        let data = processors
          .process(data, self.context.clone())
          .await
          .unwrap();
        destination.send(data, self.context.clone()).await.unwrap();
      });
  }
}
