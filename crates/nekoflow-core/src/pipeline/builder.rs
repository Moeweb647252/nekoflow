use nekoflow_macros::pipeline_builder_impls;

use super::Pipeline;
use crate::{
  destination::{Destination, Null},
  processor::{Processor, Processors},
  source::Source,
};
pub struct PipelineBuilder<S, P, D> {
  pub(crate) name: String,
  pub(crate) source: S,
  pub(crate) processors: P,
  pub(crate) destination: D,
}

impl PipelineBuilder<(), (), ()> {
  pub fn new(name: impl ToString) -> Self {
    PipelineBuilder {
      name: name.to_string(),
      source: (),
      processors: (),
      destination: (),
    }
  }

  pub fn source<S: Source>(self, source: S) -> PipelineBuilder<S, (), ()> {
    PipelineBuilder {
      name: self.name,
      source: source,
      processors: (),
      destination: (),
    }
  }
}

impl<S: Source> PipelineBuilder<S, (), ()> {
  pub fn processor<P1: Processor<Recv = S::Send>>(
    self,
    processor: P1,
  ) -> PipelineBuilder<S, (P1,), ()> {
    PipelineBuilder {
      name: self.name,
      source: self.source,
      processors: (processor.into(),),
      destination: (),
    }
  }
}

impl<S: Source<Send = P::Recv>, P: Processors, D: Destination<Recv = P::Send>>
  PipelineBuilder<S, P, D>
{
  pub fn build(self) -> Pipeline<S, P, D> {
    Pipeline {
      name: self.name,
      source: self.source,
      destination: self.destination,
      processors: self.processors,
    }
  }
}

impl<S: Source<Send = P::Recv>, P: Processors> PipelineBuilder<S, P, ()> {
  pub fn build(self) -> Pipeline<S, P, Null<P::Send>> {
    Pipeline {
      name: self.name,
      source: self.source,
      destination: Null::new(),
      processors: self.processors,
    }
  }
}

pipeline_builder_impls!(16);
