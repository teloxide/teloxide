use crate::types::{MessageEntity, User};

/// Converts an optional iterator to a flattened iterator.
pub(crate) fn flatten<I>(opt: Option<I>) -> impl Iterator<Item = I::Item>
where
    I: IntoIterator,
{
    struct Flat<I>(Option<I>);

    impl<I> Iterator for Flat<I>
    where
        I: Iterator,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.as_mut()?.next()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            match &self.0 {
                None => (0, Some(0)),
                Some(i) => i.size_hint(),
            }
        }
    }

    Flat(opt.map(<_>::into_iter))
}

pub(crate) fn mentioned_users_from_entities(
    entities: &[MessageEntity],
) -> impl Iterator<Item = &User> {
    use crate::types::MessageEntityKind::*;

    entities.iter().filter_map(|entity| match &entity.kind {
        TextMention { user } => Some(user),

        Mention
        | Hashtag
        | Cashtag
        | BotCommand
        | Url
        | Email
        | PhoneNumber
        | Bold
        | Blockquote
        | ExpandableBlockquote
        | Italic
        | Underline
        | Strikethrough
        | Spoiler
        | Code
        | Pre { language: _ }
        | TextLink { url: _ }
        | CustomEmoji { custom_emoji_id: _ } => None,
    })
}
