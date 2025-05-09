use std::task::Context;

use crate::error::Result;

pub trait Processor {
  type Send;
  type Recv;
  fn process(&self, data: Self::Recv, ctx: Context) -> Result<Self::Send>;
}
