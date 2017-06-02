use quote::{Tokens, ToTokens};

use dissect::{Context, Fragment, Interpolation};
use error::{Error, Result};

pub fn expand(context: &Context) -> Result<Tokens> {
    let fragments = &context.fragments;

    Ok(quote! { {
        extern crate interp;
        interp::Interpolator::new(|w| {
            #(#fragments)*
            Ok(())
        })
    } })
}

impl<'a> ToTokens for Fragment<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        tokens.append(match *self {
            Fragment::String(ref s) => quote!(write!(w, #s)?;),
            Fragment::Interpolation(ref i) => quote!(write!(w, "{}", #i)?;),
        })
    }
}

impl<'a> ToTokens for Interpolation<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        tokens.append("{");
        tokens.append(self.0);
        tokens.append("}");
    }
}
