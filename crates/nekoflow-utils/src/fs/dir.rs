use nekoflow_core::{context::Context, destination::Destination, error::Result, source::Source};
use std::{fs::Metadata, path::PathBuf};
use tokio::fs::{self, ReadDir};

use super::file::VirtualFile;

pub struct FileInfo {
  filename: String,
  metadata: Metadata,
  absolute_path: PathBuf,
  relative_path: PathBuf,
}

pub struct Directory {
  name: String,
  ty: DirectoryType,
  recursive: bool,
}

pub enum DirectoryType {
  Real(RealDirectory),
  Virtual(VirtualDirectory),
}

pub struct RealDirectory {
  path: PathBuf,
  recursive: bool,
  iter: Vec<ReadDir>,
}

pub struct VirtualDirectory {
  name: String,
  entries: Vec<VirtualEntry>,
}

pub enum VirtualEntry {
  Dir(VirtualDirectory),
  File(VirtualFile),
}
