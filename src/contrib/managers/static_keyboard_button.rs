use crate::contrib::views::ViewFactory;
use crate::contrib::parser::{Parser, DataWithUWC};
use crate::types::{KeyboardButton, Message};
use crate::dispatching::UpdateWithCx;

/// Represents a manager for static `KeyboardButton`s. Static means dynamic creation. It can create
/// an button with selected text, and parse incoming text messages by this text.
/// 
/// Example:
/// ```
/// use teloxide::contrib::{
///     managers::StaticKeyboardButtonManager
/// };
/// use teloxide::dummies::{update_with_cx, text_message};
/// 
/// let manager = StaticKeyboardButtonManager::new("test");
/// 
/// assert_eq!(manager.construct(()), KeyboardButton::new("test"));
/// 
/// let test = update_with_cx(text_message("test"));
/// assert_eq!(manager.parse(test).is_ok());
/// 
/// let wrong = update_with_cx(text_message("wrong"));
/// assert_eq!(manager.parse(wrong).is_err());
/// ```
#[derive(Debug)]
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