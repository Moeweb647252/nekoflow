use std::time::Instant;

pub struct Metadata {
  pub name: String,
  pub size: usize,
  pub created: Instant,
}
