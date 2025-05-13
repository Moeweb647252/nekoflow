use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{LitInt, Result};

pub fn generate_pipeline_builder_impls(input: LitInt) -> Result<TokenStream> {
  let processor_count = input.base10_parse::<usize>()?;

  let mut generated_code = proc_macro2::TokenStream::new();

  let s_ident = format_ident!("S");
  let d_ident = format_ident!("D");
  let pipeline_builder_ident = format_ident!("PipelineBuilder");
  let pipeline_ident = format_ident!("Pipeline");
  let source_trait_ident = format_ident!("Source");
  let processor_trait_ident = format_ident!("Processor");
  let destination_trait_ident = format_ident!("Destination");
  for count in 1..=processor_count {
    let mut impl_generics_params = Vec::new();
    let mut generics_params = Vec::new();
    impl_generics_params.push(quote! {#s_ident: #source_trait_ident});
    generics_params.push(quote! {#s_ident});
    for i in 1..=count {
      let pi_ident = format_ident!("P{}", i);
      impl_generics_params.push(quote! {#pi_ident: #processor_trait_ident});
      generics_params.push(quote! {#pi_ident});
    }
    let processor_method = if count != processor_count {
      let next_method_generics_ident = format_ident!("P{}", count + 1);
      let method_generics_ident = format_ident!("P{}", count);
      let mut ret_generics_params = generics_params.clone();
      ret_generics_params.push(quote! {#next_method_generics_ident});
      let mut pipeline_value_params = (0..=count)
        .map(|i| {
          let index = syn::Index::from(i);
          quote! { self.pipeline.#index }
        })
        .collect::<Vec<_>>();
      pipeline_value_params.push(quote! {processor});
      quote! {
        pub fn processor<#next_method_generics_ident: #processor_trait_ident<Recv = #method_generics_ident::Send>>(self, processor: #next_method_generics_ident) -> #pipeline_builder_ident<(#(#ret_generics_params),*)> {
          Self::set_pipeline((#(#pipeline_value_params),*))
        }
      }
    } else {
      quote! {}
    };
    let destination_method = {
      let mut ret_generics_params = generics_params.clone();
      ret_generics_params.push(quote! {#d_ident});
      let mut pipeline_value_params = (0..=count)
        .map(|i| {
          let index = syn::Index::from(i);
          quote! { self.pipeline.#index }
        })
        .collect::<Vec<_>>();
      pipeline_value_params.push(quote! {destination});
      quote! {
        pub fn destination<#d_ident: #destination_trait_ident<Recv = #s_ident::Send>>(self, destination: #d_ident) -> #pipeline_builder_ident<(#(#ret_generics_params),*)> {
          Self::set_pipeline((#(#pipeline_value_params),*))
        }
      }
    };
    generated_code.extend(quote! {
      impl<#(#impl_generics_params),*> #pipeline_builder_ident<(#(#generics_params),*)> {
        #processor_method
        #destination_method
      }

      impl<#(#impl_generics_params),*,D: #destination_trait_ident> #pipeline_builder_ident<(#(#generics_params),*,D)> {
        pub fn build(self) -> #pipeline_ident<(#(#generics_params),*,D)> {
          Self::inner_build(self)
        }
      }
    });
  }
  Ok(generated_code.into())
}
