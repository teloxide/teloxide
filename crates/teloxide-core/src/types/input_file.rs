use bytes::{Bytes, BytesMut};
use futures::{
    future::{ready, Either},
    stream,
};
use once_cell::sync::OnceCell;
use rc_box::ArcBox;
use reqwest::{multipart::Part, Body};
use serde::Serialize;
use takecell::TakeCell;
use tokio::{
    io::{AsyncRead, AsyncReadExt, ReadBuf},
    sync::watch,
};
use tokio_util::codec::{Decoder, FramedRead};

use std::{
    borrow::Cow, convert::Infallible, fmt, future::Future, io, iter, mem, path::PathBuf, pin::Pin,
    sync::Arc, task,
};

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
    Read(Read),
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
    #[must_use]
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

    /// Creates an `InputFile` from a in-memory bytes.
    ///
    /// Note: in some cases (e.g. sending the same `InputFile` multiple times)
    /// this may read the whole `impl AsyncRead` into memory.
    pub fn read(it: impl AsyncRead + Send + Unpin + 'static) -> Self {
        Self::new(Read(Read::new(Arc::new(TakeCell::new(it)))))
    }

    /// Shorthand for `Self { file_name: None, inner, id: default() }`
    /// (private because `InnerFile` is private implementation detail)
    fn new(inner: InnerFile) -> Self {
        Self { file_name: None, inner, id: OnceCell::new() }
    }

    /// Returns id of this file.
    ///
    /// This is used to coordinate with `attach://`.
    pub(crate) fn id(&self) -> &str {
        let random = || Arc::from(&*uuid::Uuid::new_v4().as_simple().encode_lower(&mut [0; 32]));
        self.id.get_or_init(random)
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
            Read(_) => f.debug_struct("Read").finish_non_exhaustive(),
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

        match self.inner {
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
                Some(Either::Right(Either::Left(ready(stream))))
            }
            Read(read) => Some(Either::Right(Either::Right(read.into_part(filename)))),
        }
    }
}

/// Adaptor for `AsyncRead` that allows clonning and converting to
/// `multipart/form-data`
#[derive(Clone)]
struct Read {
    inner: Arc<TakeCell<dyn AsyncRead + Send + Unpin>>,
    buf: Arc<OnceCell<Result<Vec<Bytes>, Arc<io::Error>>>>,
    notify: Arc<watch::Sender<()>>,
    wait: watch::Receiver<()>,
}

impl Read {
    fn new(it: Arc<TakeCell<dyn AsyncRead + Send + Unpin>>) -> Self {
        let (tx, rx) = watch::channel(());

        Self { inner: it, buf: Arc::default(), notify: Arc::new(tx), wait: rx }
    }

    pub(crate) async fn into_part(mut self, filename: Cow<'static, str>) -> Part {
        if !self.inner.is_taken() {
            let res = ArcBox::<TakeCell<dyn AsyncRead + Send + Unpin>>::try_from(self.inner);
            match res {
                // Fast/easy path: this is the only file copy, so we can just forward the underlying
                // `dyn AsyncRead` via some adaptors to reqwest.
                Ok(arc_box) => {
                    let fr = FramedRead::new(ExclusiveArcAsyncRead(arc_box), BytesDecoder);

                    let body = Body::wrap_stream(fr);
                    return Part::stream(body).file_name(filename);
                }
                // move the arc back into `self`
                Err(i) => self.inner = i,
            }
        }

        // Slow path: either wait until someone will read the whole `dyn AsyncRead` into
        // a buffer, or be the one who reads
        let body = self.into_shared_body().await;

        Part::stream(body).file_name(filename)
    }

    async fn into_shared_body(mut self) -> Body {
        match self.inner.take() {
            // Read `dyn AsyncRead` into a buffer
            Some(mut read_ref) => {
                // Chunk size, arbitrary chosen to be 1KiB
                const CHUNK: usize = 1024;

                let mut chunks = Vec::new();
                let mut bytes = BytesMut::with_capacity(CHUNK);

                let res = loop {
                    match (&mut read_ref).read_buf(&mut bytes).await {
                        // eof
                        Ok(0) if bytes.len() < bytes.capacity() => {
                            chunks.push(bytes.freeze());

                            break Ok(chunks);
                        }

                        // No space left in bytes, allocate a new chunk
                        Ok(0) => {
                            chunks.push(bytes.freeze());
                            bytes = BytesMut::with_capacity(CHUNK);
                        }

                        // keep reading into the same chunk
                        Ok(_) => {}

                        // i/o error
                        Err(err) => break Err(Arc::new(err)),
                    }
                };

                // Initialize `buf` with the result.
                // Error indicates that the `buf` was already initialized, but this can't happen
                // since we synchronize through other means.
                let r = self.buf.set(res);
                debug_assert!(r.is_ok());

                // Notify other tasks that `buf` is initialized.
                // Error indicates that there is no one to notify anymore, but we don't care.
                let _ = self.notify.send(());
            }

            // Wait until `dyn AsyncRead` is read into a buffer, if it hasn't been read yet
            None if self.buf.get().is_none() => {
                // Error indicates that the sender was dropped, by we hold `Arc<Sender>`, so
                // this can't happen
                let _ = self.wait.changed().await;
            }

            // Someone else has already initialized the buffer
            None => {}
        };

        let buf = self.buf;
        // unwrap: `OnceCell` is initialized in the match above before sending
        // notification, so at this point it's already initialized.
        match buf.get().unwrap() {
            Ok(_) => {
                // We can't use `.iter()` here, because the iterator must capture `buf`
                let mut i = 0;
                let iter = iter::from_fn(move || match buf.get().unwrap() {
                    Ok(buf) if i >= buf.len() => None,
                    Ok(buf) => {
                        let res = buf[i].clone();
                        i += 1;
                        Some(Ok::<_, Infallible>(res))
                    }
                    // We've just checked in the above match, it's `Ok(_)`
                    Err(_) => unreachable!(),
                });

                Body::wrap_stream(stream::iter(iter))
            }

            Err(err) => {
                let err = Err::<Bytes, _>(Arc::clone(err));
                Body::wrap_stream(stream::iter(iter::once(err)))
            }
        }
    }
}

/// Wrapper over an `ArcBox` that implements `AsyncRead`.
struct ExclusiveArcAsyncRead(ArcBox<TakeCell<dyn AsyncRead + Send + Unpin>>);

impl AsyncRead for ExclusiveArcAsyncRead {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> task::Poll<io::Result<()>> {
        let Self(inner) = Pin::get_mut(self);
        let read: &mut (dyn AsyncRead + Unpin) = inner.get();
        Pin::new(read).poll_read(cx, buf)
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
        let (Self::Png(input_file) | Self::Tgs(input_file) | Self::Webm(input_file)) = self;

        input_file.copy_into(into)
    }

    fn move_into(&mut self, into: &mut dyn FnMut(InputFile)) {
        let (Self::Png(input_file) | Self::Tgs(input_file) | Self::Webm(input_file)) = self;

        input_file.move_into(into)
    }
}
