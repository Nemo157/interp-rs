pub type Result<T> = ::std::result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        TODO(err: &'static str) {
            description(&err)
        }
        Syn(err: String) { description(&err) }
        Lex(err: ::proc_macro::LexError) { from() }
        Nom(err: ::nom::Err<u32>) {
            from()
        }
    }
}
