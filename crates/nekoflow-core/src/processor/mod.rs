use std::marker::PhantomData;

use crate::context::Context;
use crate::error::Result;
use nekoflow_macros::processors_impls;

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

processors_impls!(16);

pub struct FnProcessor<F, R, S> {
  pub(crate) f: F,
  _marker: PhantomData<(R, S)>,
}

impl<F, R, S> FnProcessor<F, R, S>
where
  F: Fn(R) -> S,
{
  pub fn new(f: F) -> Self {
    Self {
      f,
      _marker: PhantomData,
    }
  }
}

impl<F, R, S> From<F> for FnProcessor<F, R, S>
where
  F: Fn(R) -> S,
{
  fn from(value: F) -> Self {
    Self::new(value)
  }
}

impl<F, R, S> Processor for FnProcessor<F, R, S>
where
  F: Fn(R) -> S,
{
  type Recv = R;
  type Send = S;

  async fn process(&self, data: Self::Recv, _: Context) -> Result<Self::Send> {
    Ok((self.f)(data))
  }
}
