// use proc_macro::quote;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    match impl_derive(&ast) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn impl_derive(ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &ast.ident;
    let builder_name = format_ident!("{}Builder", struct_name);
    let gen = quote! {
    pub struct #builder_name {

    }
    impl #struct_name {
        pub fn builder()-> #builder_name{
            #builder_name{}
        }
    }};

    return Ok(gen.into());
}
