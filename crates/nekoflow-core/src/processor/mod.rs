use nekoflow_macros::processors_impls;
use crate::context::Context;
use crate::error::Result;

pub trait Processor {
  type Send;
  type Recv;
  fn process(&self, data: Self::Recv, ctx: Context) -> impl Future<Output = Result<Self::Send>>;
}

pub trait Processors {
  type Send;
  type Recv;
  fn process(&self, data: Self::Recv, ctx: Context) -> impl Future<Output = Result<Self::Send>>;
}

impl<P: Processor> Processors for P {
  type Send = P::Send;
  type Recv = P::Recv;

  async fn process(&self, data: Self::Recv, ctx: Context) -> Result<Self::Send> {
    self.process(data, ctx).await
  }
}

processors_impls!(10);