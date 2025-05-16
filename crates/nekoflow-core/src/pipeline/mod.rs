mod builder;
pub use builder::PipelineBuilder;

use crate::{destination::Destination, processor::Processors, source::Source};

pub struct Pipeline<S: Source<Send = P::Recv>, P: Processors, D: Destination<Recv = P::Send>> {
  pub(crate) name: String,
  pub(crate) source: S,
  pub(crate) destination: D,
  pub(crate) processors: P,
}

impl<P: Processors, S: Source<Send = P::Recv>, D: Destination<Recv = P::Send>> Pipeline<S, P, D> {
  pub fn source(&self) -> &S {
    &self.source
  }
  pub fn destination(&self) -> &D {
    &self.destination
  }
  pub fn processors(&self) -> &P {
    &self.processors
  }
}
