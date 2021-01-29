pub struct ParserOut<T, Rest> {
    pub data: T,
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

pub trait RecombineFrom<Parser, From, Rest> {
    fn recombine(info: ParserOut<From, Rest>) -> Self;
}
