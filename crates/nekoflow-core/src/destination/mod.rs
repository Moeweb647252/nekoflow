use crate::{context::Context, payload::Payload};

pub trait Destination {
  fn send(&mut self, payload: Payload, ctx: Context) -> Result<(), String>;
}
