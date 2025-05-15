use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{LitInt, Result};

/*
todo:
 1.refactor func
 2.add Into support
*/

// Generates implementations for `PipelineBuilder` based on the specified processor count.
// This allows for creating pipelines with a variable number of processing stages.
pub fn generate_pipeline_builder_impls(input: LitInt) -> Result<TokenStream> {
  // Parse the input literal (expected to be an integer) to determine the maximum number of processors.
  let processor_count = input.base10_parse::<usize>()?;

  // Initialize an empty token stream to accumulate the generated code.
  let mut generated_code = proc_macro2::TokenStream::new();

  // Define identifiers for commonly used types and traits.
  let s_ident = format_ident!("S");
  let d_ident = format_ident!("D");
  let pipeline_builder_ident = format_ident!("PipelineBuilder");
  let pipeline_ident = format_ident!("Pipeline");
  let source_trait_ident = format_ident!("Source");
  let processor_trait_ident = format_ident!("Processor");
  let destination_trait_ident = format_ident!("Destination");

  for cur_processor in 2..=processor_count {
    let mut impl_generics_processors_params = Vec::new();
    for i in 1..=cur_processor {

    }
    generated_code.extend(qoute! {
      
    });
  }
  Ok(generated_code.into())
}
