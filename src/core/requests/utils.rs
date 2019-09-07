use tokio::codec::FramedRead;
use std::fs::File;
use bytes::{Bytes, BytesMut};
use tokio::prelude::*;
use reqwest::r#async::multipart::Part;
use std::path::PathBuf;

struct FileDecoder;

impl tokio::codec::Decoder for FileDecoder {
    type Item = Bytes;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None)
        }
        Ok(Some(src.take().freeze()))
    }
}

pub fn file_to_part(path_to_file: &PathBuf) -> Part {
    let file = tokio::fs::File::open(path_to_file.clone())
        .map(|file| FramedRead::new(file.unwrap() /* TODO: this can cause panics */, FileDecoder))
        .flatten_stream();
    let part = Part::stream(file)
        .file_name(path_to_file.file_name().unwrap().to_string_lossy().into_owned());
    part
}