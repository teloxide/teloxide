use crate::dispatching::UpdateWithCx;

/// Struct that represent an parsed data and a raw `UpdateWithCx<Upd>`. After tests we noticed
/// that in code we often need both the `UpdateWithCx<Upd>` and parsed data. This struct was returned
/// form `Parser::parse` method.
pub struct DataWithUWC<D, U> {
    pub data: D,
    pub uwc: UpdateWithCx<U>,
}

impl<D, U> DataWithUWC<D, U> {
    pub fn new(data: D, uwc: UpdateWithCx<U>) -> Self {
        DataWithUWC { data, uwc }
    }
}

/// Parser is a trait that parses one type to another and if it fails returns input object. 
/// If parsing is ok it must return `DataWithUWC` object that contains parsed data and input
/// `UpdateWithCx<Upd>` object.
pub trait Parser {
    /// Update type that given from telegram. 
    type Update : Send + Sync + 'static;
    /// Data, parsed from `Update` object. If there are no need in parsed data, it may be `()`.
    type Output : Send + Sync + 'static;
    fn parse(&self, data: UpdateWithCx<Self::Update>) -> Result<DataWithUWC<Self::Output, Self::Update>, UpdateWithCx<Self::Update>>;
}