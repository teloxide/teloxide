//! A minimal "mock" of teloxide crate that is needed to test macros. Changes to
//! `teloxide`[`-core`] should be kept in sync with this... somehow
//!
//! This is a price for placing all crates in separate repositories.

pub use teloxide_macros as macros;

pub mod utils {
    pub mod command {
        use std::{
            error::Error,
            fmt::{self, Display, Write},
            marker::PhantomData,
        };

        pub use teloxide_macros::BotCommands;

        use crate::types::{BotCommand, Me};
        pub trait BotCommands: Sized {
            fn parse<N>(s: &str, bot_username: N) -> Result<Self, ParseError>
            where
                N: Into<String>;

            fn descriptions() -> CommandDescriptions<'static>;

            fn bot_commands() -> Vec<BotCommand>;

            fn ty() -> PhantomData<Self> {
                PhantomData
            }
        }

        pub type PrefixedBotCommand = String;
        pub type BotName = String;

        #[derive(Debug)]
        pub enum ParseError {
            TooFewArguments { expected: usize, found: usize, message: String },
            TooManyArguments { expected: usize, found: usize, message: String },

            IncorrectFormat(Box<dyn Error + Send + Sync + 'static>),

            UnknownCommand(PrefixedBotCommand),
            WrongBotName(BotName),

            Custom(Box<dyn Error + Send + Sync + 'static>),
        }

        #[derive(Debug, Clone)]
        #[allow(dead_code)]
        pub struct CommandDescriptions<'a> {
            global_description: Option<&'a str>,
            descriptions: &'a [CommandDescription<'a>],
            bot_username: Option<&'a str>,
        }

        #[derive(Debug, Clone)]
        pub struct CommandDescription<'a> {
            pub prefix: &'a str,
            pub command: &'a str,
            pub description: &'a str,
        }

        impl<'a> CommandDescriptions<'a> {
            pub fn new(descriptions: &'a [CommandDescription<'a>]) -> Self {
                Self {
                    global_description: None,
                    descriptions,
                    bot_username: None,
                }
            }

            pub fn global_description(
                self,
                global_description: &'a str,
            ) -> Self {
                Self { global_description: Some(global_description), ..self }
            }

            pub fn username(self, bot_username: &'a str) -> Self {
                Self { bot_username: Some(bot_username), ..self }
            }

            pub fn username_from_me(
                self,
                me: &'a Me,
            ) -> CommandDescriptions<'a> {
                self.username(
                    me.user
                        .username
                        .as_deref()
                        .expect("Bots must have usernames"),
                )
            }
        }

        impl Display for CommandDescriptions<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(global_description) = self.global_description {
                    f.write_str(global_description)?;
                    f.write_str("\n\n")?;
                }

                let mut write =
                    |&CommandDescription { prefix, command, description },
                     nls| {
                        if nls {
                            f.write_char('\n')?;
                        }

                        f.write_str(prefix)?;
                        f.write_str(command)?;

                        if let Some(username) = self.bot_username {
                            f.write_char('@')?;
                            f.write_str(username)?;
                        }

                        if !description.is_empty() {
                            f.write_str(" — ")?;
                            f.write_str(description)?;
                        }

                        fmt::Result::Ok(())
                    };

                if let Some(descr) = self.descriptions.first() {
                    write(descr, false)?;
                    for descr in &self.descriptions[1..] {
                        write(descr, true)?;
                    }
                }

                Ok(())
            }
        }
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
    pub struct Me {
        #[serde(flatten)]
        pub user: User,
        pub can_join_groups: bool,
        pub can_read_all_group_messages: bool,
        pub supports_inline_queries: bool,
    }
    #[serde_with_macros::skip_serializing_none]
    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
    pub struct User {
        pub id: UserId,
        pub is_bot: bool,
        pub first_name: String,
        pub last_name: Option<String>,
        pub username: Option<String>,
        pub language_code: Option<String>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub is_premium: bool,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub added_to_attachment_menu: bool,
    }

    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Serialize,
        Deserialize,
    )]
    #[serde(transparent)]
    pub struct UserId(pub u64);

    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
    pub struct BotCommand {
        pub command: String,
        pub description: String,
    }

    impl BotCommand {
        pub fn new<S1, S2>(command: S1, description: S2) -> Self
        where
            S1: Into<String>,
            S2: Into<String>,
        {
            Self { command: command.into(), description: description.into() }
        }

        pub fn command<S>(mut self, val: S) -> Self
        where
            S: Into<String>,
        {
            self.command = val.into();
            self
        }

        pub fn description<S>(mut self, val: S) -> Self
        where
            S: Into<String>,
        {
            self.description = val.into();
            self
        }
    }
}
