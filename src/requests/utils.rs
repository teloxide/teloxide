use std::{borrow::Cow, path::PathBuf};

use bytes::{Bytes, BytesMut};
use reqwest::{multipart::Part, Body};
use tokio_util::codec::{Decoder, FramedRead};

struct FileDecoder;

impl Decoder for FileDecoder {
    type Item = Bytes;
    type Error = std::io::Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }
        Ok(Some(src.split().freeze()))
    }
}

pub async fn file_to_part(path_to_file: PathBuf) -> Part {
    let file_name =
        path_to_file.file_name().unwrap().to_string_lossy().into_owned();

    let file = FramedRead::new(
        tokio::fs::File::open(path_to_file).await.unwrap(), /* TODO: this
                                                             * can
                                                             * cause panics */
        FileDecoder,
    );

    Part::stream(Body::wrap_stream(file)).file_name(file_name)
}

pub fn file_from_memory_to_part(
    data: Cow<'static, [u8]>,
    name: String,
) -> Part {
    Part::bytes(data).file_name(name)
}
