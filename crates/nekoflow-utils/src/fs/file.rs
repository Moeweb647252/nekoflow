use std::{fs::Metadata, io::Cursor};

pub struct VirtualFile {
  data: Cursor<Vec<u8>>,
  filename: String,
  metadata: Metadata,
}
