use serde::Serialize;

use crate::{
    payloads::GetUpdates,
    requests::Payload,
    types::{NonStrictVec, Update},
};

/// The fault tolerant version of [`GetUpdates`].
#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Serialize)]
#[serde(transparent)]
pub struct GetUpdatesFaultTolerant(pub GetUpdates);

impl Payload for GetUpdatesFaultTolerant {
    type Output = NonStrictVec<Update>;

    const NAME: &'static str = GetUpdates::NAME;
}
