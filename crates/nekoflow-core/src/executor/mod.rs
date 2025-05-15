use crate::pipeline::Pipeline;
use crate::{
  destination::Destination,
  processor::{Processor, Processors},
  source::Source,
};
use crate::context::Context;

pub struct Executor<P: Processors, S: Source<Send = P::Recv>, D: Destination<Recv = P::Send>> {
  pipeline: Pipeline<S, P, D>,
  context: Context
}

pub trait ExecutorTrait {
  fn execute(&mut self);
}

impl<P: Processors, S: Source<Send = P::Recv>, D: Destination<Recv = P::Send>> ExecutorTrait
  for Executor<P, S, D>
{
  fn execute(&mut self) {
    // Execute the pipeline
    let source = &mut self.pipeline.source;
    let destination = &mut self.pipeline.destination;
    let processors = &mut self.pipeline.processors;

    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
      let data = source.get(self.context.clone()).await.unwrap();
      let data = processors.process(data, self.context.clone()).await.unwrap();
      destination.send(data, self.context.clone()).await.unwrap();
    });
  }
}
