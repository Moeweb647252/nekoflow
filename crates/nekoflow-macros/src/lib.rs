mod pipeline;
mod processors;
use proc_macro::TokenStream;
use syn::{LitInt, parse_macro_input};

#[proc_macro]
pub fn pipeline_builder_impls(input: TokenStream) -> TokenStream {
  let max_processors_lit = parse_macro_input!(input as LitInt);
  match pipeline::generate_pipeline_builder_impls(max_processors_lit) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

#[proc_macro]
pub fn processors_impls(input: TokenStream) -> TokenStream {
  let max_processors_lit = parse_macro_input!(input as LitInt);
  match processors::generate_processors_impls(max_processors_lit) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}