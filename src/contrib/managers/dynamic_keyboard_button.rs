use std::marker::PhantomData;
use crate::contrib::views::ViewFactory;
use crate::types::KeyboardButton;
use std::convert::TryFrom;
use crate::contrib::parser::{Parser, DataWithUWC};
use crate::dispatching::UpdateWithCx;
use crate::prelude::Message;

pub struct Prefix {
    prefix: String,
    separator: String,
}

impl Prefix {
    pub fn new<P: Into<String>, S: Into<String>>(prefix: P, separator: S) -> Self {
        Prefix { prefix: prefix.into(), separator: separator.into() }
    }
}

pub struct DynamicKeyboardButtonManager<D> {
    prefix: Option<Prefix>,
    phantom: PhantomData<D>,
}
impl<D> DynamicKeyboardButtonManager<D> {
    pub fn new<P>(prefix: P) -> Self
        where
            P: Into<Option<Prefix>>,
    {
        DynamicKeyboardButtonManager { prefix: prefix.into(), phantom: PhantomData }
    }
}
impl<D: Into<String>> ViewFactory for DynamicKeyboardButtonManager<D> {
    type Ctx = D;
    type View = KeyboardButton;

    fn construct(&self, data: Self::Ctx) -> Self::View {
        let text = match &self.prefix {
            Some(prefix) => format!("{}{}{}", prefix.prefix, prefix.separator, data.into()),
            None => data.into()
        };
        KeyboardButton::new(text)
    }
}
impl<D: TryFrom<String> + Send + Sync + 'static> Parser for DynamicKeyboardButtonManager<D> {
    type Update = Message;
    type Output = D;

    fn parse(&self, cx: UpdateWithCx<Self::Update>) -> Result<DataWithUWC<Self::Output, Self::Update>, UpdateWithCx<Self::Update>> {
        let text = match cx.update.text() {
            Some(t) => t,
            None => return Err(cx)
        };
        let data_raw = match &self.prefix {
            Some(pref) => match text.split(&pref.separator).collect::<Vec<&str>>().as_slice() {
                [prefix, data] => match prefix == &&pref.prefix {
                    true => data,
                    false => return Err(cx)
                }
                _ => return Err(cx)
            },
            None => text
        }.to_string();
        let data = match D::try_from(data_raw) {
            Ok(d) => d,
            Err(_) => return Err(cx)
        };
        Ok(DataWithUWC::new(data, cx))
    }
}
