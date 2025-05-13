mod builder;
pub use builder::PipelineBuilder;

use crate::{destination::Destination, error::Result, processor::Processor, source::Source};

pub struct Pipeline<S: Source, P, D: Destination> {
  pub(crate) source: S,
  pub(crate) destination: D,
  pub(crate) processors: P,
}

impl<S: Source, P, D: Destination> Pipeline<S, P, D> {
  pub fn source(&mut self) -> &mut S {
    &mut self.source
  }
}
