extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TimelessPartialEq)]
pub fn partial_eq_except_timestamps(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        panic!("TimelessPartialEq can only be derived for structs with named fields");
    };

    let field_comparisons = fields.iter().filter_map(|field| {
        let ident = &field.ident;
        let field_type = &field.ty;
        if ident.as_ref().unwrap().to_string().ends_with("_at") {
            if field_type
                .to_token_stream()
                .to_string()
                .starts_with("Option")
            {
                return Some(quote! { self.#ident.is_none() && other.#ident.is_none() || self.#ident.is_some() && other.#ident.is_some()});
            }
            
            None
        } else {
            Some(quote! { &self.#ident == &other.#ident })
        }
    });

    let expanded = quote! {
        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                #(#field_comparisons)&&*
            }
        }
    };

    TokenStream::from(expanded)
}
