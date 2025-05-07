use std::task::Context;

use crate::error::Result;

trait Source {
  fn get(&mut self, ctx: Context) -> Result;
}
