use crate::context::Context;

use crate::error::Result;

pub trait Source {
  type Send;
  fn get(&mut self, ctx: Context) -> impl Future<Output = Result<Option<Self::Send>>>;
}
