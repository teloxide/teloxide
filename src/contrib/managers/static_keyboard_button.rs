use crate::contrib::views::ViewFactory;
use crate::contrib::parser::{Parser, DataWithUWC};
use crate::types::{KeyboardButton, Message};
use crate::dispatching::UpdateWithCx;

/// 
/// 
/// ```compile_fail
/// use teloxide::contrib::StaticKeyboardButtonManager;
/// use teloxide::types::{KeyboardButton, Message};
/// use teloxide::prelude::UpdateWithCx;
/// 
/// let manager = StaticKeyboardButtonManager::new("test");
/// 
/// assert_eq!(manager.construct(()), KeyboardButton::new("test"));
/// assert!(manager.parse(UpdateWithCx { bot, update: message(text: "test")}).is_ok());
/// assert!(manager.parse(UpdateWithCx { bot, update: message(text: "must fail")}).is_err());
/// ```
/// 
pub struct StaticKeyboardButtonManager {
    name: String,
}
impl StaticKeyboardButtonManager {
    pub fn new<T: Into<String>>(name: T) -> Self {
        StaticKeyboardButtonManager { name: name.into() }
    }
}
impl ViewFactory for StaticKeyboardButtonManager {
    type Ctx = ();
    type View = KeyboardButton;

    fn construct(&self, _: Self::Ctx) -> Self::View {
        KeyboardButton::new(self.name.clone())
    }
}
impl Parser for StaticKeyboardButtonManager {
    type Update = Message;
    type Output = ();

    fn parse(&self, cx: UpdateWithCx<Self::Update>) -> Result<DataWithUWC<Self::Output, Self::Update>, UpdateWithCx<Self::Update>> {
        let text = match cx.update.text() {
            Some(t) => t,
            None => return Err(cx)
        };
        match &self.name == text {
            true => Ok(DataWithUWC::new((), cx)),
            false => Err(cx)
        }
    }
}