use std::str;

use syn::{self, Expr};
use nom::{named, map, delimited, many1, alt_complete, take_until_s, tag_s, take_while_s};

// TODO: Internals of nom, should open an issue about not needing these
use nom::{tuple_parser, call, error_position, complete, tag, take_until, alt, take_while};

use crate::error::Result;

type NStr<'a> = nom::types::CompleteStr<'a>;

#[derive(Debug)]
enum IFragment<'a> {
    String(NStr<'a>),
    Interpolation(NStr<'a>),
}

pub enum Fragment {
    String(String),
    Interpolation(Expr),
}

pub struct Context {
    pub fragments: Vec<Fragment>,
}

fn a(c: char) -> bool { c != '{' }

named!(code<NStr, IFragment>, map!(take_until_s!("}"), IFragment::Interpolation));
named!(interpolation<NStr, IFragment>, delimited!(tag_s!("{"), code, tag_s!("}")));
named!(interpolate<NStr, Vec<IFragment>>, many1!(alt_complete!(interpolation | map!(take_while_s!(a), IFragment::String))));

pub fn dissect(string: &str) -> Result<Context> {
    let string = nom::types::CompleteStr(string);
    let (unparsed, parsed) = interpolate(string)?;
    println!("unparsed: {:?}", unparsed);
    println!("parsed: {:?}", parsed);
    assert!(unparsed.is_empty());

    let fragments = parsed.into_iter().map(|f| Ok(match f {
        IFragment::String(s) => Fragment::String(s.as_ref().to_owned()),
        IFragment::Interpolation(s) => Fragment::Interpolation(syn::parse_str(s.as_ref())?),
    })).collect::<Result<Vec<Fragment>>>().unwrap();
    Ok(Context { fragments: fragments })
}
