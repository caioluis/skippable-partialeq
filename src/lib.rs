//! Ignore fields ending with a specific suffix in PartialEq custom implementations.
//!
//! # Examples
//! ```
//! use timeless_partialeq::TimelessPartialEq;
//! use chrono::{DateTime, TimeZone, Utc};
//! 

//! #[derive(Debug, TimelessPartialEq)]
//! #[exclude_suffix(at, date)]
//! pub struct Post {
//!     pub id: i64,
//!     pub content: String,
//!     pub author: i32,
//!     pub creation_date: DateTime<Utc>,
//!     pub updated_at: Option<DateTime<Utc>>,
//! }
//! 

//! assert_eq!(
//!     Post {
//!         id: 1,
//!         content: "test".to_string(),
//!         author: 1,
//!         creation_date: Utc.timestamp_millis_opt(1715017040672).unwrap(),
//!         updated_at: Some(Utc.timestamp_millis_opt(1715017020672).unwrap()),
//!     },
//!     Post {
//!         id: 1,
//!         content: "test".to_string(),
//!         author: 1,
//!         creation_date: Utc::now(),
//!         updated_at: Some(Utc::now()),
//!     }
//! ) // true
//! ```
//! 
//! # About the crate
//! This crate was made to solve a very specific problem: assert the equality of two objects despite the timestamp differences. It was also made so that I could study proc macros.
//! However, just after a day after publishing it, I realized that it can be broader than just timestamps.
//! 
//! I will not make a commitment into iterating this quickly, but it is in my plans to expand the scope of the crate.

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(TimelessPartialEq, attributes(exclude_suffix))]
pub fn partial_eq_except_timestamps(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let mut args: Vec<String> = Vec::new();
    
    for attr in ast.attrs.iter() {
        if attr.path().is_ident("exclude_suffix") {
            let meta_list = attr.meta.require_list();
            if let Ok(meta_list) = meta_list {
                for arg in meta_list.tokens.to_token_stream() {
                    let arg = arg.to_string();
                    if arg != "," {
                        args.push(arg);
                    }
                } 
            }
        }
    }

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
        let field = ident.as_ref().unwrap().to_string();
        if args.is_empty() {
            args.push("at".to_string());
        }
        if args.iter().any(|arg| field.ends_with(&format!("_{}", arg))) {
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
