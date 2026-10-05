use crate::{
    payloads,
    requests::Payload,
    types::{InputFile, InputFileLike, InputMedia, InputPaidMedia, InputSticker},
};

/// Payloads that need to be sent as `multipart/form-data` because they contain
/// files inside.
pub trait MultipartPayload: Payload {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile));

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile));
}

impl MultipartPayload for payloads::SendPaidMedia {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter().flat_map(InputPaidMedia::files).for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter_mut().flat_map(InputPaidMedia::files_mut).for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::SendMediaGroup {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter().flat_map(InputMedia::files).for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter_mut().flat_map(InputMedia::files_mut).for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::EditMessageMedia {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.files().for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.files_mut().for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::EditMessageMediaInline {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.files().for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.files_mut().for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::CreateNewStickerSet {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.stickers
            .iter()
            .for_each(|InputSticker { sticker: f, .. }: &InputSticker| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.stickers
            .iter_mut()
            .for_each(|InputSticker { sticker: f, .. }: &mut InputSticker| f.move_into(into))
    }
}

impl MultipartPayload for payloads::SendPoll {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        if let Some(media) = &self.media {
            media.files().for_each(|file| file.copy_into(into));
        }
        if let Some(media) = &self.explanation_media {
            media.files().for_each(|file| file.copy_into(into));
        }
        for option in &self.options {
            if let Some(media) = &option.media {
                media.files().for_each(|file| file.copy_into(into));
            }
        }
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        if let Some(media) = &mut self.media {
            media.files_mut().for_each(|file| file.move_into(into));
        }
        if let Some(media) = &mut self.explanation_media {
            media.files_mut().for_each(|file| file.move_into(into));
        }
        for option in &mut self.options {
            if let Some(media) = &mut option.media {
                media.files_mut().for_each(|file| file.move_into(into));
            }
        }
    }
}

impl MultipartPayload for payloads::PostStory {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.content.files().for_each(|file| file.copy_into(into));
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.content.files_mut().for_each(|file| file.move_into(into));
    }
}

impl MultipartPayload for payloads::EditStory {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.content.files().for_each(|file| file.copy_into(into));
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.content.files_mut().for_each(|file| file.move_into(into));
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        payloads::{EditStory, PostStory, SendPoll, SendRichMessage},
        requests::{MultipartPayload, MultipartRequest, Requester},
        types::{
            BusinessConnectionId, ChatId, InputFile, InputMedia, InputMediaAnimation,
            InputMediaDocument, InputMediaLivePhoto, InputMediaPhoto, InputMediaSticker,
            InputMediaVideo, InputPollMedia, InputPollOption, InputPollOptionMedia,
            InputRichMessage, InputRichMessageMedia, InputRichMessageMediaContent,
            InputStoryContent, InputStoryContentPhoto, InputStoryContentVideo, Seconds, StoryId,
            UserId,
        },
        Bot,
    };

    fn file() -> InputFile {
        InputFile::memory(vec![1_u8])
    }

    fn populated_poll() -> SendPoll {
        let options = vec![
            InputPollOption::new("sticker").media(InputPollOptionMedia::Sticker(
                InputMediaSticker { media: file(), emoji: None },
            )),
            InputPollOption::new("live")
                .media(InputPollOptionMedia::LivePhoto(InputMediaLivePhoto::new(file(), file()))),
        ];

        let mut payload = SendPoll::new(ChatId(1), "question", options);
        payload.media = Some(InputPollMedia::Video(
            InputMediaVideo::new(file()).thumbnail(file()).cover(file()),
        ));
        payload.explanation_media =
            Some(InputPollMedia::Animation(InputMediaAnimation::new(file()).thumbnail(file())));
        payload
    }

    #[test]
    fn send_poll_collects_every_nested_attachment() {
        let mut payload = populated_poll();

        let mut copied = 0;
        payload.copy_files(&mut |_| copied += 1);
        assert_eq!(copied, 8);

        let mut moved = 0;
        payload.move_files(&mut |_| moved += 1);
        assert_eq!(moved, 8);
    }

    #[test]
    fn send_poll_uses_multipart_request() {
        fn assert_multipart(_: MultipartRequest<SendPoll>) {}

        let request = Bot::new("token").send_poll(
            ChatId(1),
            "question",
            vec![InputPollOption::new("one"), InputPollOption::new("two")],
        );

        assert_multipart(request);
    }

    #[test]
    fn ephemeral_message_media_uses_multipart_request() {
        fn assert_multipart(_: MultipartRequest<crate::payloads::EditEphemeralMessageMedia>) {}

        assert_multipart(Bot::new("token").edit_ephemeral_message_media(
            UserId(1),
            UserId(2),
            3,
            InputMedia::Photo(InputMediaPhoto::new(file())),
        ));
    }

    #[test]
    fn story_payloads_collect_their_nested_attachment() {
        let mut post = PostStory::new(
            BusinessConnectionId("business".to_owned()),
            InputStoryContent::Photo(InputStoryContentPhoto { photo: file() }),
            Seconds::from_seconds(6 * 3600),
        );
        let mut copied = 0;
        post.copy_files(&mut |_| copied += 1);
        assert_eq!(copied, 1);
        let mut moved = 0;
        post.move_files(&mut |_| moved += 1);
        assert_eq!(moved, 1);

        let mut edit = EditStory::new(
            BusinessConnectionId("business".to_owned()),
            StoryId(1),
            InputStoryContent::Video(InputStoryContentVideo {
                video: file(),
                duration: None,
                cover_frame_timestamp: None,
                is_animation: None,
            }),
        );
        let mut copied = 0;
        edit.copy_files(&mut |_| copied += 1);
        assert_eq!(copied, 1);
        let mut moved = 0;
        edit.move_files(&mut |_| moved += 1);
        assert_eq!(moved, 1);
    }

    #[test]
    fn story_methods_use_multipart_requests() {
        fn assert_post(_: MultipartRequest<PostStory>) {}
        fn assert_edit(_: MultipartRequest<EditStory>) {}

        let bot = Bot::new("token");
        assert_post(bot.post_story(
            BusinessConnectionId("business".to_owned()),
            InputStoryContent::Photo(InputStoryContentPhoto { photo: file() }),
            Seconds::from_seconds(6 * 3600),
        ));
        assert_edit(bot.edit_story(
            BusinessConnectionId("business".to_owned()),
            StoryId(1),
            InputStoryContent::Video(InputStoryContentVideo {
                video: file(),
                duration: None,
                cover_frame_timestamp: None,
                is_animation: None,
            }),
        ));
    }
    #[test]
    fn send_rich_message_collects_nested_attachments_and_uses_multipart() {
        fn assert_multipart(_: MultipartRequest<SendRichMessage>) {}

        let rich = InputRichMessage::html(r#"<img src="tg://photo?id=cover">"#).media([
            InputRichMessageMedia::new(
                "cover",
                InputRichMessageMediaContent::Photo(crate::types::InputMediaPhoto::new(file())),
            ),
        ]);
        let mut payload = SendRichMessage::new(ChatId(1), rich.clone());

        let mut copied = 0;
        payload.copy_files(&mut |_| copied += 1);
        assert_eq!(copied, 1);

        let mut moved = 0;
        payload.move_files(&mut |_| moved += 1);
        assert_eq!(moved, 1);

        assert_multipart(Bot::new("token").send_rich_message(ChatId(1), rich));
    }

    #[test]
    fn send_rich_message_collects_top_level_document_attachments() {
        fn assert_multipart(_: MultipartRequest<SendRichMessage>) {}

        let rich = InputRichMessage::html(r#"<img src="tg://document?id=file">"#).media([
            InputRichMessageMedia::new(
                "file",
                InputRichMessageMediaContent::Document(
                    InputMediaDocument::new(file()).thumbnail(file()),
                ),
            ),
        ]);
        let payload = SendRichMessage::new(ChatId(1), rich.clone());

        let mut copied = 0;
        payload.copy_files(&mut |_| copied += 1);
        assert_eq!(copied, 2);

        assert_multipart(Bot::new("token").send_rich_message(ChatId(1), rich));
    }
}
