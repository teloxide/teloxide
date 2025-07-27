use serde::{Deserialize, Serialize};

/// Identifier of a story.
#[derive(Clone, Copy)]
#[derive(Debug, derive_more::Display)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct StoryId(pub u64);

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that `StoryId` is serialized as the underlying integer
    #[test]
    fn deser() {
        let story_id = S { id: StoryId(17) };
        let json = r#"{"id":17}"#;

        #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
        struct S {
            id: StoryId,
        }

        assert_eq!(serde_json::to_string(&story_id).unwrap(), json);
        assert_eq!(story_id, serde_json::from_str(json).unwrap());
    }
}
