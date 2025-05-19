/*
todo:
  1.wrap a new error type that can store error position
*/

pub type Result<T = ()> = std::result::Result<T, NekoflowError>;

#[derive(Debug)]
pub enum NekoflowError {
  Anyhow(anyhow::Error),
}

impl<T> From<T> for NekoflowError
where
  T: Into<anyhow::Error>,
{
  fn from(value: T) -> Self {
    Self::Anyhow(value.into())
  }
}
