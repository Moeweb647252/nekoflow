use std::marker::PhantomData;

use crate::context::Context;
use crate::error::Result;

pub trait Destination {
  type Recv;
  fn recv(&mut self, payload: Self::Recv, ctx: Context) -> impl Future<Output = Result>;
}

pub struct Null<T> {
  _marker: PhantomData<T>,
}

impl<T> Null<T> {
  pub fn new() -> Self {
    Self {
      _marker: PhantomData,
    }
  }
}

impl<T> Destination for Null<T> {
  type Recv = T;

  async fn recv(&mut self, _: Self::Recv, _: Context) -> Result {
    Ok(())
  }
}
