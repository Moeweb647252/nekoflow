use crate::{destination::Destination, processor::Processor, source::Source};

pub struct Pipeline<P> {
  pub(crate) pipeline: P,
}

pub struct PipelineBuilder<P> {
  pub(crate) pipeline: P,
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

impl<S: Source, P: Processor> PipelineBuilder<(S, P)> {
  pub fn destination<D: Destination<Recv = P::Send>>(
    self,
    destination: D,
  ) -> PipelineBuilder<(S, P, D)> {
    PipelineBuilder {
      pipeline: (self.pipeline.0, self.pipeline.1, destination),
    }
  }
}

impl<S: Source, P: Processor, D: Destination> PipelineBuilder<(S, P, D)> {
  pub fn build(self) -> Pipeline<(S, P, D)> {
    Pipeline {
      pipeline: self.pipeline,
    }
  }
}

impl<S: Source, P: Processor> PipelineBuilder<(S, P)> {
  pub fn processor<P2: Processor<Recv = P::Send>>(
    self,
    processor: P2,
  ) -> PipelineBuilder<(S, P, P2)> {
    PipelineBuilder {
      pipeline: (self.pipeline.0, self.pipeline.1, processor),
    }
  }
}

impl<S: Source, P: Processor, P2: Processor> PipelineBuilder<(S, P, P2)> {
  pub fn destination<D: Destination<Recv = P2::Send>>(
    self,
    destination: D,
  ) -> PipelineBuilder<(S, P, P2, D)> {
    PipelineBuilder {
      pipeline: (
        self.pipeline.0,
        self.pipeline.1,
        self.pipeline.2,
        destination,
      ),
    }
  }
}

impl<S: Source, P: Processor, P2: Processor, D: Destination> PipelineBuilder<(S, P, P2, D)> {
  pub fn build(self) -> Pipeline<(S, P, P2, D)> {
    Pipeline {
      pipeline: self.pipeline,
    }
  }
}

impl<S: Source, P: Processor, P2: Processor> PipelineBuilder<(S, P, P2)> {
  pub fn processor<P3: Processor<Recv = P2::Send>>(
    self,
    processor: P3,
  ) -> PipelineBuilder<(S, P, P2, P3)> {
    PipelineBuilder {
      pipeline: (self.pipeline.0, self.pipeline.1, self.pipeline.2, processor),
    }
  }
}

impl<S: Source, P: Processor, P2: Processor, P3: Processor> PipelineBuilder<(S, P, P2, P3)> {
  pub fn destination<D: Destination<Recv = P3::Send>>(
    self,
    destination: D,
  ) -> PipelineBuilder<(S, P, P2, P3, D)> {
    PipelineBuilder {
      pipeline: (
        self.pipeline.0,
        self.pipeline.1,
        self.pipeline.2,
        self.pipeline.3,
        destination,
      ),
    }
  }
}

impl<S: Source, P: Processor, P2: Processor, P3: Processor, D: Destination>
  PipelineBuilder<(S, P, P2, P3, D)>
{
  pub fn build(self) -> Pipeline<(S, P, P2, P3, D)> {
    Pipeline {
      pipeline: self.pipeline,
    }
  }
}
