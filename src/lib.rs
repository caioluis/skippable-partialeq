//! Ignore fields in PartialEq custom implementations.
//!
//! # Examples
//! ```
//! use skippable_partialeq::SkippablePartialEq;
//! use chrono::{DateTime, TimeZone, Utc};
//! 

//! #[derive(Debug, SkippablePartialEq)]
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
//! You can also skip specific fields that do not follow a pattern using the `#[skip]` attribute above the fields you want to ignore:
//! 
//! ```
//! use skippable_partialeq::SkippablePartialEq;
//! use chrono::{DateTime, TimeZone, Utc};
//! 
//! #[derive(Debug, SkippablePartialEq)]
//! pub struct Post {
//!     pub id: i64,
//!     pub content: String,
//!     pub author: i32,
//!     #[skip]
//!     pub creation_date: DateTime<Utc>,
//! }

//! assert_eq!(
//!     Post {
//!         id: 1,
//!         content: "test".to_string(),
//!         author: 1,
//!         creation_date: Utc.timestamp_millis_opt(1715017040672).unwrap(),
//!     },
//!     Post {
//!         id: 1,
//!         content: "test".to_string(),
//!         author: 1,
//!         creation_date: Utc::now(),
//!     }
//! )
//! ```

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(SkippablePartialEq, attributes(exclude_suffix, skip))]
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
        panic!("SkippablePartialEq can only be derived for structs with named fields");
    };


    if !fields.iter().any(|field| field.attrs.iter().any(|attr| attr.path().is_ident("skip"))) && args.is_empty() {
        panic!("SkippablePartialEq needs arguments to know what fields to skip");
    }
        

    let field_comparisons = fields.iter().filter_map(|field| {
        let has_specific_skip = field.attrs.iter().any(|attr| attr.path().is_ident("skip"));

        if has_specific_skip {
            return None
        } else {
            let ident = &field.ident;
            let field_type = &field.ty;
            let field_name = ident.as_ref().unwrap().to_string();
    
            if args.iter().any(|arg| field_name.ends_with(&format!("_{}", arg))) {
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
