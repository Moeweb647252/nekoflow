use crate::error::Result;
use crate::payload::Payload;
pub trait Processor {
  fn process(&self, data: Payload) -> Result<Payload>;
}
