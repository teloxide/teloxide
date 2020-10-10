use crate::contrib::parser::{Parser, DataWithUWC};
use crate::prelude::UpdateWithCx;
use crate::types::Message;
use std::marker::PhantomData;

pub struct DynamicCommandParserBuilder<T: CommandDataParse> {
    prefix: Option<String>,
    separator: Option<String>,
    command: String,
    phantom: PhantomData<T>,
}
impl<T: CommandDataParse> DynamicCommandParserBuilder<T> {
    pub fn new<C: Into<String>>(command: C) -> Self {
        Self {
            prefix: None,
            separator: None,
            command: command.into(),
            phantom: PhantomData
        }
    }
    pub fn prefix<P: Into<String>>(mut self, prefix: P) -> Self {
        self.prefix = Some(prefix.into());
        self
    }
    pub fn separator<S: Into<String>>(mut self, separator: S) -> Self {
        self.separator = Some(separator.into());
        self
    }
    pub fn build(self) -> DynamicCommandParser<T> {
        let prefix = self.prefix.unwrap_or("/".to_string());
        let separator = self.separator.unwrap_or(" ".to_string());
        
        DynamicCommandParser { 
            command: format!(
                "{}{}",
                prefix,
                self.command
            ),
            separator,
            phantom: PhantomData
        }
    }
}

pub struct DynamicCommandParser<T: CommandDataParse> {
    command: String,
    separator: String,
    phantom: PhantomData<T>,
}

impl<T: CommandDataParse + Send + Sync + 'static> Parser for DynamicCommandParser<T> {
    type Update = Message;
    type Output = T;

    fn parse(&self, cx: UpdateWithCx<Self::Update>) -> Result<DataWithUWC<Self::Output, Self::Update>, UpdateWithCx<Self::Update>> {
        let text = match cx.update.text() {
            Some(t) => t,
            None => return Err(cx),
        };
        let data = match text.splitn(2, &self.separator).collect::<Vec<&str>>().as_slice() {
            [prefix, data] => match prefix == &&self.command {
                true => data.to_string(),
                false => return Err(cx),
            }
            _ => return Err(cx),
        };
        match T::try_parse(data, self.separator.as_str()) {
            Ok(d) => Ok(DataWithUWC::new(d, cx)),
            Err(_) => Err(cx)
        }
    }
}

pub trait CommandDataParse : Sized {
    fn try_parse(data: String, separator: &str) -> Result<Self, ()>;
}

macro_rules! impl_parse {
    ($($t:ident),*) => {
        $(impl CommandDataParse for $t {
            fn try_parse(data: String, _: &str) -> Result<Self, ()> {
                data.parse().map_err(|_| ())
            }
        })*
    }
}

impl_parse!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, String);

impl<A, B> CommandDataParse for (A, B)
    where 
        A: CommandDataParse,
        B: CommandDataParse,
{
    fn try_parse(data: String, separator: &str) -> Result<Self, ()> {
        match data.splitn(2, separator).collect::<Vec<&str>>().as_slice() {
            [a, b] => {
                let a = A::try_parse(a.to_string(), separator)?;
                let b = B::try_parse(b.to_string(), separator)?;
                Ok((a, b))
            }
            _ => Err(())
        }
    }
}

impl<A, B, C> CommandDataParse for (A, B, C)
    where
        A: CommandDataParse,
        B: CommandDataParse,
        C: CommandDataParse,
{
    fn try_parse(data: String, separator: &str) -> Result<Self, ()> {
        match data.splitn(3, separator).collect::<Vec<&str>>().as_slice() {
            [a, b, c] => {
                let a = A::try_parse(a.to_string(), separator)?;
                let b = B::try_parse(b.to_string(), separator)?;
                let c = C::try_parse(c.to_string(), separator)?;
                Ok((a, b, c))
            }
            _ => Err(())
        }
    }
}

impl<A, B, C, D> CommandDataParse for (A, B, C, D)
    where
        A: CommandDataParse,
        B: CommandDataParse,
        C: CommandDataParse,
        D: CommandDataParse,
{
    fn try_parse(data: String, separator: &str) -> Result<Self, ()> {
        match data.splitn(4, separator).collect::<Vec<&str>>().as_slice() {
            [a, b, c, d] => {
                let a = A::try_parse(a.to_string(), separator)?;
                let b = B::try_parse(b.to_string(), separator)?;
                let c = C::try_parse(c.to_string(), separator)?;
                let d = D::try_parse(d.to_string(), separator)?;
                Ok((a, b, c, d))
            }
            _ => Err(())
        }
    }
}
