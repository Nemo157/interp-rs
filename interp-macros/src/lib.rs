#![feature(proc_macro)]

#[macro_use] extern crate nom;
#[macro_use] extern crate quick_error;
#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate syn;

mod dissect;
mod error;
mod expand;

use proc_macro::TokenStream;

use error::{Error, Result};

#[proc_macro]
pub fn interp(input: TokenStream) -> TokenStream {
    fn inner(input: String) -> Result<TokenStream> {
        let tokens = syn::parse_token_trees(&input).map_err(Error::Syn)?;
        let context = dissect::dissect(&tokens)?;
        let expanded = expand::expand(&context)?;
        Ok(expanded.parse()?)
    }
    inner(input.to_string()).expect("Error expanding macro")
}
