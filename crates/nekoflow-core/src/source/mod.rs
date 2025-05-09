use std::task::Context;

use crate::error::Result;

pub trait Source {
  type Send;
  fn get(&mut self, ctx: Context) -> Result<Self::Send>;
}
