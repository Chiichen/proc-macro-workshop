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
    let builder_name = &format_ident!("{}Builder", struct_name);
    let builder_struct_feild = generate_builder_struct_feilds(ast)?;
    let buidler_struct_setters = generate_builder_setters(ast)?;
    let build = generate_builder_operation(ast)?;
    let gen = quote! {
    pub struct #builder_name {
        #builder_struct_feild
    }
    impl #builder_name{
        #buidler_struct_setters
    }
    impl #struct_name {
        #build
    }};

    return Ok(gen.into());
}

fn generate_builder_struct_feilds(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ret: proc_macro2::TokenStream;
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let idents: Vec<_> = data_struct.fields.iter().map(|f| &f.ident).collect();
            let types: Vec<_> = data_struct.fields.iter().map(|f| &f.ty).collect();
            ret = quote::quote!(
                #(#idents: std::option::Option<#types>),*
            );
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    }

    return Ok(ret);
}

fn generate_builder_setters(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut ret: proc_macro2::TokenStream = Default::default();
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let idents: Vec<_> = data_struct.fields.iter().map(|f| &f.ident).collect();
            let types: Vec<_> = data_struct.fields.iter().map(|f| &f.ty).collect();

            for (ident, ftype) in idents.iter().zip(types.iter()) {
                let single_method = quote! {
                    fn #ident(&mut self, #ident: #ftype) -> &mut Self {
                        self.#ident = std::option::Option::Some(#ident);
                        self
                    }
                };
                ret.extend(single_method);
            }
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    }
    return Ok(ret);
}

fn generate_builder_operation(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ret: proc_macro2::TokenStream;
    let struct_name = &ast.ident;
    let builder_name = &format_ident!("{}Builder", struct_name);
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let idents: Vec<_> = data_struct.fields.iter().map(|f| &f.ident).collect();
            ret = quote! {
                pub fn builder()->#builder_name{
                    return #builder_name{
                        #(#idents : None),*
                    }
                }
            };
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    }
    return Ok(ret);
}
