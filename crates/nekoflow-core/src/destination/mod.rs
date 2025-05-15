use crate::context::Context;
use crate::error::Result;

pub trait Destination {
  type Recv;
  fn send(&mut self, payload: Self::Recv, ctx: Context) -> impl Future<Output=Result>;
}
