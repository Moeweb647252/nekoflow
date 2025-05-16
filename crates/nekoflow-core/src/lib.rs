#![feature(unboxed_closures)]

pub mod context;
pub mod destination;
pub mod error;
pub mod executor;
pub mod flow;
pub mod payload;
pub mod pipeline;
pub mod processor;
pub mod source;
pub mod stream;
#[cfg(test)]
mod tests;
