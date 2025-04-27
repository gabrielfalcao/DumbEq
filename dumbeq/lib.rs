#![feature(proc_macro_hygiene)]
#![feature(proc_macro_span)]
//! # DumbEq
//!
//! dumb implementation of [`std::cmp::PartialEq`] and [`std::cmp::Eq`]
//!
//! DumbEq is always false.
//!
//! Example
//! ```
//! use dumbeq::*;
//!
//! #[derive(DumbEq, Debug)]
//! pub struct Same;
//!
//! assert!(Same != Same, "not the same");
//! ```

extern crate proc_macro;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(DumbEq)]
pub fn dumb_eq_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = match syn::parse(input).map_err(|e| Into::<proc_macro::TokenStream>::into(e.to_compile_error())) {
        Ok(ast) => ast,
        Err(ast) => return ast
    };
    let name = ast.ident;
    quote! {
        impl std::cmp::PartialEq for #name {
            fn eq(&self, other: &#name) -> bool {
                false
            }
        }
        impl std::cmp::Eq for #name {}
    }
    .into()
}
