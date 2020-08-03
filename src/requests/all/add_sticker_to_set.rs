use crate::{
    net,
    requests::form_builder::FormBuilder,
    types::{MaskPosition, True},
    Bot,
};

use crate::{
    requests::{RequestWithFile, ResponseResult},
    types::StickerType,
};

/// Use this method to add a new sticker to a set created by the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#addstickertoset).
#[derive(Debug, Clone)]
pub struct AddStickerToSet {
    bot: Bot,
    user_id: i32,
    name: String,
    sticker_type: StickerType,
    emojis: String,
    mask_position: Option<MaskPosition>,
}

#[async_trait::async_trait]
impl RequestWithFile for AddStickerToSet {
    type Output = True;

    async fn send(self) -> tokio::io::Result<ResponseResult<True>> {
        let builder =
            FormBuilder::new().add_text("user_id", &self.user_id).add_text("name", &self.name);

        let builder = match &self.sticker_type {
            StickerType::Png(file) => builder.add_input_file("png_sticker", &file),
            StickerType::Tgs(file) => builder.add_input_file("tgs_sticker", &file),
        }
        .await?
        .add_text("emojis", &self.emojis)
        .add_text("mask_position", &self.mask_position);

        Ok(net::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "addStickerToSet",
            builder.build(),
        )
        .await)
    }
}

impl AddStickerToSet {
    pub(crate) fn new<N, E>(
        bot: Bot,
        user_id: i32,
        name: N,
        sticker_type: StickerType,
        emojis: E,
    ) -> Self
    where
        N: Into<String>,
        E: Into<String>,
    {
        Self {
            bot,
            user_id,
            name: name.into(),
            sticker_type,
            emojis: emojis.into(),
            mask_position: None,
        }
    }

    /// User identifier of sticker set owner.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    /// Sticker set name.
    pub fn name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.name = val.into();
        self
    }

    pub fn sticker_type(mut self, val: StickerType) -> Self {
        self.sticker_type = val;
        self
    }

    /// One or more emoji corresponding to the sticker.
    pub fn emojis<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.emojis = val.into();
        self
    }

    /// A JSON-serialized object for position where the mask should be placed on
    /// faces.
    pub fn mask_position(mut self, val: MaskPosition) -> Self {
        self.mask_position = Some(val);
        self
    }
}
