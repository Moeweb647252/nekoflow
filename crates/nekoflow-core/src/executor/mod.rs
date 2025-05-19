use std::sync::Arc;

use tokio::sync::RwLock;

use crate::context::Context;
use crate::error::NekoflowError;
use crate::pipeline::Pipeline;
use crate::{destination::Destination, processor::Processors, source::Source};

pub struct Executor<S: Source<Send = P::Recv>, P: Processors, D: Destination<Recv = P::Send>> {
  pipeline: Pipeline<S, P, D>,
  context: Context,
  state: Arc<RwLock<ExecutorState>>,

  config: ExecutorConfig,
}

impl<S: Source<Send = P::Recv>, P: Processors, D: Destination<Recv = P::Send>> Executor<S, P, D> {
  pub fn new(pipeline: Pipeline<S, P, D>, context: Context) -> Self {
    Executor {
      pipeline,
      context,
      config: Default::default(),
      state: Arc::new(RwLock::new(ExecutorState::Init)),
    }
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
    {
      let mut state = self.state.blocking_write();
      *state = ExecutorState::Running;
    }

    tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .unwrap()
      .block_on(async {
        while let Some(data) = source.get(self.context.clone()).await.unwrap() {
          let data = processors
            .process(data, self.context.clone())
            .await
            .unwrap();
          destination.recv(data, self.context.clone()).await.unwrap();
        }
      });
  }
}

#[derive(Clone, Default)]
pub struct ExecutorConfig {
  pub paralle: bool,
}

pub enum ExecutorState {
  Init,
  Idle,
  Running,
  Success,
  Failed(NekoflowError),
}
