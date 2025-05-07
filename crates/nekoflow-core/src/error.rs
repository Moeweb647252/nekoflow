use thiserror::Error;

pub type Result<T = ()> = std::result::Result<T, NekoflowError>;

#[derive(Error, Debug)]
pub enum NekoflowError {}
