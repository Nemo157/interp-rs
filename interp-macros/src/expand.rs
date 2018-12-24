use quote::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;

use crate::dissect::{Context, Fragment};
use crate::error::Result;

pub fn expand(context: &Context) -> Result<TokenStream> {
    let fragments = &context.fragments;

    Ok(quote! { {
        extern crate interp;
        interp::Interpolator::new(|w| {
            #(#fragments)*
            Ok(())
        })
    } })
}

impl ToTokens for Fragment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            Fragment::String(ref s) => {
                quote!(write!(w, #s)?;).to_tokens(tokens);
            }
            Fragment::Interpolation(ref e) => {
                quote!(write!(w, "{}", #e)?;).to_tokens(tokens);
            }
        }
    }
}
