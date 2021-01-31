/// Output of the [`Parser::parse`] method.
///
/// [`Parser::parse`]: TODO
pub struct ParserOut<T, Rest> {
    /// Result of parsing.
    pub data: T,
    /// Rest data needed to restore the initial data.
    pub rest: Rest,
}

impl<T, Rest> ParserOut<T, Rest> {
    pub fn new(data: T, rest: Rest) -> Self {
        ParserOut { data, rest }
    }

    pub fn into_inner(self) -> (T, Rest) {
        (self.data, self.rest)
    }
}

/// Trait is used to parse some data `T` into some other data `U` with
/// possibility of recombine it.
pub trait Parser<From, To, Rest> {
    fn parse(&self, from: From) -> Result<ParserOut<To, Rest>, From>;
}

impl<F, From, To, Rest> Parser<From, To, Rest> for F
where
    F: Fn(From) -> Result<ParserOut<To, Rest>, From>,
    From: RecombineFrom<F, To, Rest>,
{
    fn parse(&self, from: From) -> Result<ParserOut<To, Rest>, From> {
        self(from)
    }
}

/// Trait is used to recombine output of the [`Parser`] to the input.
///
/// You need to store the [`Parser`] from which you parse the data because
/// `RecombineFrom` require it by first parameter (except in cases when
/// `RecombineFrom` is unique for the type).
///
/// [`Parser`]: TODO
pub trait RecombineFrom<Parser, From, Rest> {
    fn recombine(info: ParserOut<From, Rest>) -> Self;
}
