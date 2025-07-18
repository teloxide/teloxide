//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{BusinessConnectionId, True};

impl_payload! {
    /// Changes the username of a managed business account. Requires the _can_change_username_ business bot right. Returns _true_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetBusinessAccountUsername (SetBusinessAccountUsernameSetters) => True {
        required {
            /// Unique identifier of the business connection
            pub business_connection_id: BusinessConnectionId,
        }
        optional {
            /// The new value of the username for the business account; 0-32 characters
            pub username: String [into],
        }
    }
}
