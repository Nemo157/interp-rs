use std::str;
use std::str::FromStr;

use nom::IResult;
use syn::{Lit, Token, TokenTree};

use error::{Error, Result};

#[derive(Debug)]
pub enum Fragment<'a> {
    String(&'a str),
    Interpolation(Interpolation<'a>),
}

#[derive(Debug)]
pub struct Interpolation<'a>(pub &'a str);

#[derive(Debug)]
pub struct Context<'a> {
    pub fragments: Vec<Fragment<'a>>,
}

fn a(c: char) -> bool { c != '{' }

named!(code<&str, Fragment>, map!(map!(take_until_s!("}"), Interpolation), Fragment::Interpolation));
named!(interpolation<&str, Fragment>, delimited!(tag_s!("{"), code, tag_s!("}")));
named!(interpolate<&str, Vec<Fragment>>, many1!(alt_complete!(interpolation | map!(take_while_s!(a), Fragment::String))));

pub fn dissect(tokens: &[TokenTree]) -> Result<Context> {
    if tokens.len() != 1 {
        return Err(Error::TODO("support this"));
    }

    let string = match tokens[0] {
        TokenTree::Token(Token::Literal(Lit::Str(ref s, _))) => s,
        _ => { return Err(Error::TODO("support this")); },
    };

    let fragments = match interpolate(string.as_str()) {
        IResult::Done(unparsed, parsed) => {
            println!("unparsed: {:?}", unparsed);
            println!("parsed: {:?}", parsed);
            assert!(unparsed.is_empty());
            parsed
        }
        IResult::Error(err) => return Err(Error::Nom(err)),
        IResult::Incomplete(_) => return Err(Error::TODO("incomplete")),
    };

    Ok(Context { fragments: fragments })
}
