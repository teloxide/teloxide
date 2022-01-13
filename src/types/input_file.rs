use bytes::{Bytes, BytesMut};
use futures::{
    future::{ready, Either},
    stream,
};
use once_cell::sync::OnceCell;
use reqwest::{multipart::Part, Body};
use serde::Serialize;
use tokio_util::codec::{Decoder, FramedRead};

use std::{borrow::Cow, fmt, future::Future, io, mem, path::PathBuf, sync::Arc};

use crate::types::InputSticker;

/// This object represents the contents of a file to be uploaded.
///
/// [The official docs](https://core.telegram.org/bots/api#inputfile).
#[derive(Debug, Clone)]
pub struct InputFile {
    id: OnceCell<Arc<str>>,
    file_name: Option<Cow<'static, str>>,
    inner: InnerFile,
}

#[derive(Clone)]
enum InnerFile {
    File(PathBuf),
    Bytes(bytes::Bytes),
    Url(url::Url),
    FileId(String),
}

use InnerFile::*;

impl InputFile {
    /// Creates an `InputFile` from an url.
    ///
    /// Notes:
    /// - When sending by URL the target file must have the correct MIME type
    ///   (e.g., `audio/mpeg` for [`SendAudio`], etc.).
    /// - In [`SendDocument`], sending by URL will currently only work for
    ///   `GIF`, `PDF` and `ZIP` files.
    /// - To use [`SendVoice`], the file must have the type audio/ogg and be no
    ///   more than 1MB in size. 1-20MB voice notes will be sent as files.
    /// - Other configurations may work but we can't guarantee that they will.
    ///
    /// [`SendAudio`]: crate::payloads::SendAudio
    /// [`SendDocument`]: crate::payloads::SendDocument
    /// [`SendVoice`]: crate::payloads::SendVoice
    pub fn url(url: url::Url) -> Self {
        Self::new(Url(url))
    }

    /// Creates an `InputFile` from a file id.
    ///
    /// File id can be obtained from
    ///
    /// Notes:
    /// - It is not possible to change the file type when resending by file id.
    ///   I.e. a video can't be sent as a photo, a photo can't be sent as a
    ///   document, etc.
    /// - It is not possible to resend thumbnails.
    /// - Resending a photo by file id will send all of its [sizes].
    /// - file id is unique for each individual bot and can't be transferred
    ///   from one bot to another.
    /// - file id uniquely identifies a file, but a file can have different
    ///   valid file_ids even for the same bot.
    ///
    /// [sizes]: crate::types::PhotoSize
    pub fn file_id(file_id: impl Into<String>) -> Self {
        Self::new(FileId(file_id.into()))
    }

    /// Creates an `InputFile` from a file path.
    pub fn file(path: impl Into<PathBuf>) -> Self {
        Self::new(File(path.into()))
    }

    /// Creates an `InputFile` from a in-memory bytes.
    pub fn memory(data: impl Into<bytes::Bytes>) -> Self {
        Self::new(Bytes(data.into()))
    }

    /// Set the file name for this file.
    pub fn file_name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.file_name = Some(name.into());
        self
    }

    /// Shorthand for `Self { file_name: None, inner, id: default() }`
    /// (private because `InnerFile` iы private implementation detail)
    fn new(inner: InnerFile) -> Self {
        Self {
            file_name: None,
            inner,
            id: OnceCell::new(),
        }
    }

    /// Returns id of this file.
    ///
    /// This is used to coordinate with `attach://`.
    pub(crate) fn id(&self) -> &str {
        // FIXME: remove extra alloc
        self.id
            .get_or_init(|| uuid::Uuid::new_v4().to_string().into())
    }

    /// Returns `true` if this file needs an attachment i.e. it's not a file_id
    /// or url that can be serialized without any additional multipart parts.
    pub(crate) fn needs_attach(&self) -> bool {
        !matches!(self.inner, Url(_) | FileId(_))
    }

    /// Takes this file out.
    ///
    /// **Note**: this replaces `self` with a dummy value, this function should
    /// only be used when the file is about to get dropped.
    pub(crate) fn take(&mut self) -> Self {
        mem::replace(self, InputFile::file_id(String::new()))
    }

    /// Returns an attach string for `multipart/form-data` in the form of
    /// `"attach://{id}"` if this file should be uploaded via
    /// `multipart/form-data`, or the value if it may be uploaded in any way (ie
    /// it's an URL or file id).
    fn attach_or_value(&self) -> String {
        match &self.inner {
            Url(url) => url.as_str().to_owned(),
            FileId(file_id) => file_id.clone(),
            _ => {
                const PREFIX: &str = "attach://";

                let id = self.id();
                let mut s = String::with_capacity(PREFIX.len() + id.len());
                s += PREFIX;
                s += id;

                s
            }
        }
    }

    /// Takes the file name or tries to guess it based on file name in the path
    /// if `File.0`. Returns an empty string if couldn't guess.
    fn take_or_guess_filename(&mut self) -> Cow<'static, str> {
        self.file_name.take().unwrap_or_else(|| match &self.inner {
            File(path_to_file) => match path_to_file.file_name() {
                Some(name) => Cow::Owned(name.to_string_lossy().into_owned()),
                None => Cow::Borrowed(""),
            },
            _ => Cow::Borrowed(""),
        })
    }
}

impl fmt::Debug for InnerFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            File(path) => f.debug_struct("File").field("path", path).finish(),
            Bytes(bytes) if f.alternate() => f.debug_tuple("Memory").field(bytes).finish(),
            Bytes(_) => f.debug_struct("Memory").finish_non_exhaustive(),
            Url(url) => f.debug_tuple("Url").field(url).finish(),
            FileId(file_id) => f.debug_tuple("FileId").field(file_id).finish(),
        }
    }
}

impl Serialize for InputFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.attach_or_value().serialize(serializer)
    }
}

// internal api

impl InputFile {
    pub(crate) fn into_part(mut self) -> Option<impl Future<Output = Part>> {
        let filename = self.take_or_guess_filename();

        let file_part = match self.inner {
            // Url and FileId are serialized just as strings, they don't need additional parts
            Url(_) | FileId(_) => None,

            File(path_to_file) => {
                let fut = async {
                    let body = match tokio::fs::File::open(path_to_file).await {
                        Ok(file) => {
                            let file = FramedRead::new(file, BytesDecoder);

                            Body::wrap_stream(file)
                        }
                        Err(err) => {
                            // explicit type needed for `Bytes: From<?T>` in `wrap_stream`
                            let err = Err::<Bytes, _>(err);
                            Body::wrap_stream(stream::iter([err]))
                        }
                    };

                    Part::stream(body).file_name(filename)
                };

                Some(Either::Left(fut))
            }
            Bytes(data) => {
                let stream = Part::stream(data).file_name(filename);
                Some(Either::Right(ready(stream)))
            }
        };

        file_part
    }
}

struct BytesDecoder;

impl Decoder for BytesDecoder {
    type Item = Bytes;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }
        Ok(Some(src.split().freeze()))
    }
}

/// An internal trait that is used in expansion of `impl_payload!` used to work
/// with input-file-like things (`InputFile` itself, `Option<InputFile>`,
/// `InputSticker`)
pub(crate) trait InputFileLike {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile));

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile));
}

impl InputFileLike for InputFile {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        into(self.clone())
    }

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        into(self.take())
    }
}

impl InputFileLike for Option<InputFile> {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        if let Some(this) = self {
            this.copy_into(into)
        }
    }

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        if let Some(this) = self {
            this.move_into(into)
        }
    }
}

impl InputFileLike for InputSticker {
    fn copy_into(&self, into: &mut dyn FnMut(InputFile)) {
        let (InputSticker::Png(input_file) | InputSticker::Tgs(input_file)) = self;

        input_file.copy_into(into)
    }

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        let (InputSticker::Png(input_file) | InputSticker::Tgs(input_file)) = self;

        input_file.move_into(into)
    }
}
