use crate::{
    dispatching::{core::Guard, handlers::messages::message_parser::MessageParser},
    types,
    types::Message,
};

impl<UpdateParser, ParserT, Err> MessageParser<UpdateParser, ParserT, Err> {
    pub fn with_id(self, guard: impl Guard<i32> + 'static) -> Self {
        self.with_guard(move |message: &Message| guard.check(&message.id))
    }

    pub fn with_date(self, guard: impl Guard<i32> + 'static) -> Self {
        self.with_guard(move |message: &Message| guard.check(&message.date))
    }

    pub fn with_chat(self, guard: impl Guard<types::Chat> + 'static) -> Self {
        self.with_guard(move |message: &Message| guard.check(&message.chat))
    }

    pub fn with_chat_id(self, guard: impl Guard<i64> + 'static) -> Self {
        self.with_guard(move |message: &Message| guard.check(&message.chat.id))
    }

    pub fn with_via_bot(self, guard: impl Guard<types::User> + 'static) -> Self {
        self.with_guard(move |message: &Message| match &message.via_bot {
            Some(bot) => guard.check(bot),
            None => false,
        })
    }

    pub fn with_from(self, guard: impl Guard<types::User> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.from() {
            Some(user) => guard.check(user),
            None => false,
        })
    }

    pub fn with_forward_from(self, guard: impl Guard<types::ForwardedFrom> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.forward_from() {
            Some(user) => guard.check(user),
            None => false,
        })
    }

    pub fn with_forward_from_chat(self, guard: impl Guard<types::Chat> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.forward_from_chat() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn with_forward_from_message_id(self, guard: impl Guard<i32> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.forward_from_message_id() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn with_forward_signature(self, guard: impl Guard<str> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.forward_signature() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn with_forward_date(self, guard: impl Guard<i32> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.forward_date() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn with_text(self, guard: impl Guard<str> + 'static) -> Self {
        self.with_guard(move |message: &Message| match message.text() {
            Some(text) => guard.check(text),
            None => false,
        })
    }
}

impl<UpdateParser, ParserT, Err> MessageParser<UpdateParser, ParserT, Err> {
    pub fn or_with_id(self, guard: impl Guard<i32> + 'static) -> Self {
        self.or(move |message: &Message| guard.check(&message.id))
    }

    pub fn or_with_date(self, guard: impl Guard<i32> + 'static) -> Self {
        self.or(move |message: &Message| guard.check(&message.date))
    }

    pub fn or_with_chat(self, guard: impl Guard<types::Chat> + 'static) -> Self {
        self.or(move |message: &Message| guard.check(&message.chat))
    }

    pub fn or_with_chat_id(self, guard: impl Guard<i64> + 'static) -> Self {
        self.or(move |message: &Message| guard.check(&message.chat.id))
    }

    pub fn or_with_via_bot(self, guard: impl Guard<types::User> + 'static) -> Self {
        self.or(move |message: &Message| match &message.via_bot {
            Some(bot) => guard.check(bot),
            None => false,
        })
    }

    pub fn or_with_from(self, guard: impl Guard<types::User> + 'static) -> Self {
        self.or(move |message: &Message| match message.from() {
            Some(user) => guard.check(user),
            None => false,
        })
    }

    pub fn or_with_forward_from(self, guard: impl Guard<types::ForwardedFrom> + 'static) -> Self {
        self.or(move |message: &Message| match message.forward_from() {
            Some(user) => guard.check(user),
            None => false,
        })
    }

    pub fn or_with_forward_from_chat(self, guard: impl Guard<types::Chat> + 'static) -> Self {
        self.or(move |message: &Message| match message.forward_from_chat() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn or_with_forward_from_message_id(self, guard: impl Guard<i32> + 'static) -> Self {
        self.or(move |message: &Message| match message.forward_from_message_id() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn or_with_forward_signature(self, guard: impl Guard<str> + 'static) -> Self {
        self.or(move |message: &Message| match message.forward_signature() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn or_with_forward_date(self, guard: impl Guard<i32> + 'static) -> Self {
        self.or(move |message: &Message| match message.forward_date() {
            Some(chat) => guard.check(chat),
            None => false,
        })
    }

    pub fn or_with_text(self, guard: impl Guard<str> + 'static) -> Self {
        self.or(move |message: &Message| match message.text() {
            Some(text) => guard.check(text),
            None => false,
        })
    }
}
