use std::str;

use nom::IResult;
use syn::{self, Expr};

use error::{Error, Result};

#[derive(Debug)]
enum IFragment<'a> {
    String(&'a str),
    Interpolation(&'a str),
}

pub enum Fragment {
    String(String),
    Interpolation(Expr),
}

pub struct Context {
    pub fragments: Vec<Fragment>,
}

fn a(c: char) -> bool { c != '{' }

named!(code<&str, IFragment>, map!(take_until_s!("}"), IFragment::Interpolation));
named!(interpolation<&str, IFragment>, delimited!(tag_s!("{"), code, tag_s!("}")));
named!(interpolate<&str, Vec<IFragment>>, many1!(alt_complete!(interpolation | map!(take_while_s!(a), IFragment::String))));

pub fn dissect(string: &str) -> Result<Context> {
    let fragments = match interpolate(string) {
        IResult::Done(unparsed, parsed) => {
            println!("unparsed: {:?}", unparsed);
            println!("parsed: {:?}", parsed);
            assert!(unparsed.is_empty());
            parsed
        }
        IResult::Error(err) => return Err(Error::Nom(err)),
        IResult::Incomplete(_) => return Err(Error::TODO("incomplete")),
    };

    let fragments = fragments.into_iter().map(|f| Ok(match f {
        IFragment::String(s) => Fragment::String(s.to_owned()),
        IFragment::Interpolation(s) => Fragment::Interpolation(syn::parse_str(s)?),
    })).collect::<Result<Vec<Fragment>>>().unwrap();
    Ok(Context { fragments: fragments })
}
