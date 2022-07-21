use teloxide_core::types::{ChatId, Update};

/// Default distribution key for dispatching.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct DefaultKey(ChatId);

pub(crate) fn default_distribution_function(update: &Update) -> Option<DefaultKey> {
    update.chat().map(|c| c.id).map(DefaultKey)
}
