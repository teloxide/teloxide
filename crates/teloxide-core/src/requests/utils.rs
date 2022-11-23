use bytes::{Bytes, BytesMut};
use tokio_util::codec::Decoder;

struct FileDecoder;

impl Decoder for FileDecoder {
    type Item = Bytes;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }
        Ok(Some(src.split().freeze()))
    }
}
