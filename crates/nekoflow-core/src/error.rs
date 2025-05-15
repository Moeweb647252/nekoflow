use thiserror::Error;

/*
todo:
  1.wrap a new error type that can store error position
*/

pub type Result<T = ()> = std::result::Result<T, NekoflowError>;

#[derive(Error, Debug)]
pub enum NekoflowError {}
