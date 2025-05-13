use super::Pipeline;
use crate::{destination::Destination, processor::Processor, source::Source};
pub struct PipelineBuilder<P> {
  pub(crate) pipeline: P,
}

impl<P> PipelineBuilder<P> {
  pub(crate) fn set_pipeline<P2>(pipeline: P2) -> PipelineBuilder<P2> {
    PipelineBuilder { pipeline }
  }

  pub(crate) fn inner_build(self) -> Pipeline<P> {
    Pipeline {
      pipeline: self.pipeline,
    }
  }
}

impl PipelineBuilder<()> {
  pub fn new() -> Self {
    PipelineBuilder { pipeline: () }
  }
}

impl PipelineBuilder<()> {
  pub fn source<S: Source>(self, source: S) -> PipelineBuilder<S> {
    PipelineBuilder { pipeline: source }
  }
}

impl<S: Source> PipelineBuilder<S> {
  pub fn destination<D: Destination<Recv = S::Send>>(
    self,
    destination: D,
  ) -> PipelineBuilder<(S, D)> {
    PipelineBuilder {
      pipeline: (self.pipeline, destination),
    }
  }
}

impl<S: Source, D: Destination> PipelineBuilder<(S, D)> {
  pub fn build(self) -> Pipeline<(S, D)> {
    Pipeline {
      pipeline: self.pipeline,
    }
  }
}

impl<S: Source> PipelineBuilder<S> {
  pub fn processor<P: Processor<Recv = S::Send>>(self, processor: P) -> PipelineBuilder<(S, P)> {
    PipelineBuilder {
      pipeline: (self.pipeline, processor),
    }
  }
}

use nekoflow_macros::pipeline_builder_impls;
pipeline_builder_impls!(3);
