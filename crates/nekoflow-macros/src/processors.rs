use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::LitInt;

/*
todo: 
 1.add error handling in process_body
 */

pub fn generate_processors_impls(input: LitInt) -> syn::Result<TokenStream> {
  let processor_count = input.base10_parse::<usize>()?;
  if processor_count <= 1 {
    return Err(syn::Error::new(
      proc_macro2::Span::call_site(),
      "Processor count must be greater than 1",
    ));
  }
  let processor_trait_ident = syn::Ident::new("Processor", Span::call_site());
  let processors_trait_ident = syn::Ident::new("Processors", Span::call_site());
  let source_trait_ident = syn::Ident::new("Source", Span::call_site());
  let context_ident = syn::Ident::new("Context", Span::call_site());
  let result_ident = syn::Ident::new("Result", Span::call_site());

  let mut generated_stream = TokenStream::new();

  for cur_count in 2..=processor_count {
    let mut impl_generics_params = Vec::new();
    let mut processors_idents = Vec::new();
    let first_processor_ident = syn::Ident::new("P1", Span::call_site());
    let last_processor_ident = format_ident!("P{}", cur_count);
    let mut destruct_idents = Vec::new();
    let mut process_body = TokenStream::new();
    for i in 1..=cur_count {
      let pi_ident = format_ident!("P{}", i);
      let destruct_i_ident = format_ident!("p{}", i);
      processors_idents.push(pi_ident.clone());
      destruct_idents.push(destruct_i_ident.clone());
      impl_generics_params.push(if i == 1 {
        quote! {#pi_ident: #processor_trait_ident}
      }else {
        let pi_prev_ident = format_ident!("P{}", i-1);
        quote! {#pi_ident: #processor_trait_ident<Recv = #pi_prev_ident::Send>}
      } );
      let body_data_ident = syn::Ident::new("data", Span::call_site());
      process_body.extend(if i==cur_count {
        quote! {
          #destruct_i_ident.process(#body_data_ident, ctx).await
        }
      } else {
        quote! {
          let #body_data_ident = #destruct_i_ident.process(#body_data_ident, ctx.clone()).await?;
        }
      })
    }
    generated_stream.extend(quote! {
      impl<#(#impl_generics_params),*> #processors_trait_ident for (#(#processors_idents),*) {
        type Recv=#first_processor_ident::Recv;
        type Send=#last_processor_ident::Send;
        async fn process(&self, data: Self::Recv, ctx: #context_ident) -> #result_ident<Self::Send> {
          let (#(#destruct_idents),*) = self;
          #process_body
        }
      }
    });
  }

  Ok(generated_stream.into())
}
