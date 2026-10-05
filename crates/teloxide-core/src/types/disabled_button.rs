use serde::{Deserialize, Serialize};

/// A disabled inline or rich-message button which does nothing.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct DisabledButton {}
