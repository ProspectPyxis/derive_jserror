//! This library provides a convenience derive macro for `impl From<ErrorType> for JsValue`.
//!
//! # Example
//!
//! This example uses the `thiserror` crate to simplify the example, but it should work for any
//! type that implements [`std::error::Error`].
//!
//! ```rust
//! use derive_jserror::JsError;
//! use wasm_bindgen::prelude::*;
//! use thiserror::Error;
//!
//! #[derive(Error, JsError, Debug)]
//! #[error("test error type")]
//! struct TestError;
//!
//! #[wasm_bindgen]
//! pub fn func_returns_result() -> Result<(), TestError> {
//!     Err(TestError)
//! }
//! ```
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(JsError)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl From<#ident> for wasm_bindgen::JsValue {
            fn from(error: #ident) -> Self {
                wasm_bindgen::JsError::from(error).into()
            }
        }
    };
    output.into()
}
