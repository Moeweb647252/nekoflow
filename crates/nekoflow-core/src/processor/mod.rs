use crate::context::Context;
use crate::error::Result;

pub trait Processor {
  type Send;
  type Recv;
  fn process(&self, data: Self::Recv, ctx: Context) -> Result<Self::Send>;
}

pub trait Processors {
  type Send;
  type Recv;
  fn process(&self, data: Self::Recv, ctx: Context) -> Result<Self::Send>;
}

impl<P1: Processor, P2: Processor<Recv = P1::Send>> Processors for (P1, P2) {
  type Send = P2::Send;
  type Recv = P1::Recv;

  fn process(&self, data: Self::Recv, ctx: Context) -> Result<Self::Send> {
    let (p1, p2) = self;
    let data = p1.process(data, ctx.clone())?;
    p2.process(data, ctx)
  }
}
