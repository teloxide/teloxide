use crate::{
    payloads,
    requests::Payload,
    types::{InputFile, InputFileLike, InputMedia, InputSticker},
};

/// Payloads that need to be sent as `multipart/form-data` because they contain
/// files inside.
pub trait MultipartPayload: Payload {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile));

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile));
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
