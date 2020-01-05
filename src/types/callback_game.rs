/// A placeholder, currently holds no information. Use [@BotFather](https://t.me/botfather) to set up your game.
///
/// [The official docs](https://core.telegram.org/bots/api#callbackgame).
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CallbackGame;
