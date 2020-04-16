use serde::{Deserialize, Serialize};

/// This object represents a dice with random value from 1 to 6.
/// (Yes, we're aware of the “proper” singular of die.
/// But it's awkward, and we decided to help it change. One dice at a time!).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Dice {
    /// Value of the dice, 1-6
    pub value: u32,
}
