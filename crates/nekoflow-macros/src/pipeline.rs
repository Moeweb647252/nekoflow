use proc_macro2::{Ident, Span, TokenStream};
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
  let mut generated_code = TokenStream::new();

  // Define identifiers for commonly used types and traits.
  let s_ident = Ident::new("S", Span::call_site());
  let d_ident = Ident::new("D", Span::call_site());
  let pipeline_builder_ident = Ident::new("PipelineBuilder", Span::call_site());
  let pipeline_ident = Ident::new("Pipeline", Span::call_site());
  let source_trait_ident = Ident::new("Source", Span::call_site());
  let processor_trait_ident = Ident::new("Processor", Span::call_site());
  let destination_trait_ident = Ident::new("Destination", Span::call_site());

  for cur_processor in 2..=processor_count + 1 {
    let mut impl_generics_processors_params = Vec::new();
    let last_processor_ident = format_ident!("P{}", cur_processor);
    let processor_ident = format_ident!("P{}", cur_processor - 1);
    let mut next_processors_idents = Vec::new();
    let mut processors_idents = Vec::new();
    let mut processor_items = Vec::new();
    for i in 1..=cur_processor {
      let pi_ident = format_ident!("P{}", i);
      next_processors_idents.push(pi_ident.clone());
      if i != cur_processor {
        let i_sub_one = syn::Index::from(i - 1);
        processors_idents.push(pi_ident.clone());
        processor_items.push(quote! {self.processors.#i_sub_one});
        impl_generics_processors_params.push(if i == 1 {
          quote! {#pi_ident: #processor_trait_ident<Recv = #s_ident::Send>}
        } else {
          let pi_prev_ident = format_ident!("P{}", i - 1);
          quote! {#pi_ident: #processor_trait_ident<Recv = #pi_prev_ident::Send>}
        });
      }
    }
    let processor_method = if cur_processor == processor_count + 1 {
      quote! {}
    } else {
      quote! {
        pub fn processor<#last_processor_ident: #processor_trait_ident<Recv=#processor_ident::Send>>(
          self,
          processor: #last_processor_ident) -> #pipeline_builder_ident<#s_ident, (#(#next_processors_idents),*,), ()> {
            #pipeline_builder_ident {
              name: self.name,
              source: self.source,
              processors: (#(#processor_items),*, processor),
              destination: (),
            }
        }
      }
    };
    let destination_method = quote! {
      pub fn destination<#d_ident: #destination_trait_ident<Recv = #processor_ident::Send>>(
        self,
        destination: #d_ident) -> #pipeline_builder_ident<#s_ident, (#(#processors_idents),*,), #d_ident> {
          #pipeline_builder_ident {
            name: self.name,
            source: self.source,
            processors: (#(#processor_items),*,),
            destination,
          }
      }
    };
    generated_code.extend(quote! {
      impl<#s_ident: #source_trait_ident, #(#impl_generics_processors_params),*> #pipeline_builder_ident<#s_ident,(#(#processors_idents),*,), ()> {
        #processor_method
        #destination_method
      }
    });
  }
  Ok(generated_code.into())
}

#[cfg(test)]
mod tests {
  use proc_macro2::Span;
  use syn::LitInt;

  #[test]
  fn test_generate_pipeline_builder_impls() {
    let input = LitInt::new("2", Span::call_site());
    let result = super::generate_pipeline_builder_impls(input);
    assert!(result.is_ok());
    let generated_code = result.unwrap();
    println!("{}", generated_code.to_string());
  }
}
