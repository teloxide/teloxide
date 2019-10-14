use std::path::PathBuf;

use bytes::{Bytes, BytesMut};
use reqwest::{multipart::Part, Body};
use tokio::{codec::FramedRead, prelude::*};

struct FileDecoder;

impl tokio::codec::Decoder for FileDecoder {
    type Item = Bytes;
    type Error = std::io::Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }
        Ok(Some(src.take().freeze()))
    }
}

pub fn file_to_part(path_to_file: PathBuf) -> Part {
    let file_name = path_to_file
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let file = tokio::fs::File::open(path_to_file)
        .map(|file| {
            FramedRead::new(
                file.unwrap(), /* TODO: this can cause panics */
                FileDecoder,
            )
        })
        .flatten_stream();
    Part::stream(Body::wrap_stream(file)).file_name(file_name)
}
