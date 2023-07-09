use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(JsError)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl From< #ident > for wasm_bindgen::JsValue {
            fn from(error: #ident ) -> Self {
                wasm_bindgen::JsError::from(error).into()
            }
        }
    };
    output.into()
}
