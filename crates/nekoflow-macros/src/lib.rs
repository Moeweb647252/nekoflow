use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{LitInt, Result, Type, parse_macro_input};

fn generate_pipeline_builder_impls(input: LitInt) -> Result<TokenStream> {
  let processor_count = input.base10_parse::<usize>()?;

  let mut generated_code = proc_macro2::TokenStream::new();

  // 通用标识符
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
      let method_generics_ident = format_ident!("P{}", count + 1);
      quote! {
        pub fn processor<>()
      }
    } else {
      quote! {}
    };
    generated_code.extend(quote! {
      impl<#(#impl_generics_params),*> #pipeline_builder_ident<#(#generics_params),*> {

      }
    });
  }
  todo!()
}

#[proc_macro]
pub fn pipeline_builder_impls(input: TokenStream) -> TokenStream {
  let max_processors_lit = parse_macro_input!(input as LitInt);
  match generate_pipeline_builder_impls(max_processors_lit) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}
