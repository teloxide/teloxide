use std::string::FromUtf16Error;

use teloxide_core::types::MessageEntity;

#[derive(Debug)]
pub enum InvokeEntityError {
    IndexOutOfBound,
    FromUTF16(FromUtf16Error),
}

impl From<FromUtf16Error> for InvokeEntityError {
    fn from(e: FromUtf16Error) -> Self {
        Self::FromUTF16(e)
    }
}

impl std::fmt::Display for InvokeEntityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvokeEntityError::IndexOutOfBound => f.write_str("index out of bound"),
            InvokeEntityError::FromUTF16(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for InvokeEntityError {}

fn invoke_entity_from_utf16(
    text: &[u16],
    entity: &MessageEntity,
) -> Result<String, InvokeEntityError> {
    let start = entity.offset;
    let end = entity.offset + entity.length;
    if text.len() < end {
        Err(InvokeEntityError::IndexOutOfBound)
    } else {
        String::from_utf16(&text[start..end]).map_err(Into::into)
    }
}
pub fn invoke_entity(text: &str, entity: &MessageEntity) -> Result<String, InvokeEntityError> {
    let text: Vec<_> = text.encode_utf16().collect();
    invoke_entity_from_utf16(text.as_slice(), entity)
}
pub fn invoke_entities(
    text: &str,
    entities: &Vec<MessageEntity>,
) -> Result<Vec<String>, InvokeEntityError> {
    let text16: Vec<_> = text.encode_utf16().collect();
    let slice = text16.as_slice();
    let mut result = Vec::new();
    for e in entities {
        result.push(invoke_entity_from_utf16(slice, e)?);
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::MessageEntityKind::*;
    fn test_invoke(text: &str, entity: MessageEntity, value: &str) {
        assert_eq!(invoke_entity(text, &entity).unwrap(), value.to_owned());
    }
    #[test]
    fn en_tag() {
        test_invoke(
            "some tag: #some_tag",
            MessageEntity { kind: Hashtag, offset: 10, length: 9 },
            "#some_tag",
        );
    }

    #[test]
    fn ru_tag() {
        test_invoke(
            "какой-то тэг #просто_тэг",
            MessageEntity { kind: Hashtag, offset: 13, length: 11 },
            "#просто_тэг",
        );
    }

    #[test]
    fn tag_with_smile() {
        test_invoke(
            "smile 😁 перед тэгом #bugoga",
            MessageEntity { kind: Hashtag, offset: 21, length: 7 },
            "#bugoga",
        );
    }

    #[test]
    fn multiple_entities() {
        let text = "быба";
        let entities = vec![
            MessageEntity { kind: Strikethrough, offset: 0, length: 1 },
            MessageEntity { kind: Bold, offset: 1, length: 1 },
            MessageEntity { kind: Italic, offset: 2, length: 1 },
            MessageEntity { kind: Code, offset: 3, length: 1 },
        ];
        assert_eq!(invoke_entities(text, &entities).unwrap(), vec!["б", "ы", "б", "а"])
    }
}
