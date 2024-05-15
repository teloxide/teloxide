use serde::{Deserialize, Serialize};

/// A message identifiers.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageIds(pub Vec<i32>);
