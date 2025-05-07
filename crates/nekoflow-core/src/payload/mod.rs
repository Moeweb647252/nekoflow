use bytes::Bytes;

pub struct Payload {
  pub name: String,
  pub content: Content,
  pub metadata: Metadata,
}

pub enum Content {
  Bytes(Bytes),
  Empty,
}

pub struct Metadata {
  pub source: Option<String>,
  pub timestamp: u64,
}
