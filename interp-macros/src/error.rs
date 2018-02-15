pub type Result<T> = ::std::result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        TODO(err: &'static str) {
            description(&err)
        }
        Syn(err: ::syn::synom::ParseError) { from() }
        Lex(err: ::proc_macro::LexError) { from() }
        Nom(err: ::nom::Err<u32>) {
            from()
        }
    }
}
