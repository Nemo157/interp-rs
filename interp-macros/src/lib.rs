#![feature(extern_crate_item_prelude)]

extern crate proc_macro;

mod dissect;
mod error;
mod expand;

use proc_macro::TokenStream;

use crate::error::{Error, Result};

#[proc_macro]
pub fn interp(input: TokenStream) -> TokenStream {
    fn inner(input: TokenStream) -> Result<TokenStream> {
        let string: syn::LitStr = syn::parse(input).map_err(Error::Syn)?;
        let string = string.value();
        let context = dissect::dissect(&string)?;
        let expanded = expand::expand(&context)?;
        println!("expanded: {:#?}", expanded);
        println!("expanded stream: {:#?}", proc_macro2::TokenStream::from(expanded.clone()));
        Ok(expanded.into())
    }
    inner(input).expect("Error expanding macro")
}
