use quick_error::quick_error;

pub type Result<T> = ::std::result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Syn(err: syn::parse::Error) { from() }
        Lex(err: proc_macro::LexError) { from() }
        Nom(err: nom::Err<String>) { from() }
    }
}

impl<'a> From<nom::Err<nom::types::CompleteStr<'a>>> for Error {
    fn from(err: nom::Err<nom::types::CompleteStr<'a>>) -> Self {
        Error::from(match err {
            nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
            nom::Err::Error(nom::Context::Code(s, kind)) => {
                nom::Err::Error(nom::Context::Code(s.as_ref().to_owned(), kind))
            },
            nom::Err::Failure(nom::Context::Code(s, kind)) => {
                nom::Err::Failure(nom::Context::Code(s.as_ref().to_owned(), kind))
            },
        })
    }
}
