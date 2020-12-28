use crate::{
    serde_multipart::unserializers::{InputFileUnserializer, StringUnserializer},
    types::InputFile,
    RequestError,
};
use futures::{
    future::{ready, BoxFuture},
    stream::FuturesUnordered,
    FutureExt, StreamExt, TryStreamExt,
};
use reqwest::multipart::{Form, Part};
use serde::{
    ser,
    ser::{Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant},
    Serialize, Serializer,
};
use std::{fmt, fmt::Display, io};

#[derive(Debug, derive_more::From)]
pub(crate) enum Error {
    Custom(String),
    TopLevelNotStruct,
    InputFileUnserializer(crate::serde_multipart::unserializers::UnserializerError),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(s) => write!(f, "Custom serde error: {}", s),
            Self::TopLevelNotStruct => write!(f, "Multipart supports only structs at top level"),
            Self::InputFileUnserializer(inner) => {
                write!(f, "Error while unserializing input file: {}", inner)
            }
            Self::Io(inner) => write!(f, "Io error: {}", inner),
            Self::Json(inner) => write!(f, "Json (de)serialization error: {}", inner),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<Error> for RequestError {
    fn from(err: Error) -> Self {
        match err {
            Error::Io(ioerr) => RequestError::Io(ioerr),
            // this should be ok since we don't write request those may trigger errors and
            // Error is internal.
            _ => unreachable!(
                "we don't create requests those fail to serialize (if you see this, open an issue \
                 :|)"
            ),
        }
    }
}

pub(crate) struct MultipartTopLvlSerializer {}

impl Serializer for MultipartTopLvlSerializer {
    type Ok = <MultipartSerializer as SerializeStruct>::Ok;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = MultipartMapSerializer; // for `serde(flatten)` (e.g.: in CreateNewStickerSet)
    type SerializeStruct = MultipartSerializer;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MultipartMapSerializer {
            parts: vec![],
            files: vec![],
            key: None,
        })
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(MultipartSerializer::new())
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::TopLevelNotStruct)
    }
}

pub(crate) struct MultipartSerializer {
    parts: Vec<(&'static str, Part)>, // TODO: Array vecs
    files: Vec<(String, InputFile)>,
}

impl MultipartSerializer {
    fn new() -> Self {
        Self {
            parts: Vec::new(),
            files: vec![],
        }
    }
}

impl SerializeStruct for MultipartSerializer {
    type Ok = BoxFuture<'static, io::Result<Form>>; // impl Future<Output = io::Result<Form>>
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let (part, file) = value.serialize(PartSerializer {})?;
        self.parts.push((key, part));
        self.files.extend(file);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let form = self
            .parts
            .into_iter()
            .fold(Form::new(), |acc, (key, value)| acc.part(key, value));

        if self.files.is_empty() {
            //Ok(Either::Left(ready(Ok(form))))
            Ok(Box::pin(ready(Ok(form))))
        } else {
            let fut = self
                .files
                .into_iter()
                .map(|(k, f)| f.into_part().map(move |p| (k, p)))
                .collect::<FuturesUnordered<_>>()
                .map(Ok)
                .try_fold(form, |acc, (k, p)| async { Ok(acc.part(k, p?)) });

            //Ok(Either::Right(fut))
            Ok(Box::pin(fut))
        }
    }
}

pub(crate) struct MultipartMapSerializer {
    parts: Vec<(String, Part)>, // TODO: Array vecs
    files: Vec<(String, InputFile)>,
    key: Option<String>,
}

impl SerializeMap for MultipartMapSerializer {
    type Ok = <MultipartSerializer as SerializeStruct>::Ok;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.key = Some(key.serialize(StringUnserializer)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = self.key.take().unwrap();

        let (part, file) = value.serialize(PartSerializer {})?;
        self.parts.push((key, part));
        self.files.extend(file);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let form = self
            .parts
            .into_iter()
            .fold(Form::new(), |acc, (key, value)| acc.part(key, value));

        if self.files.is_empty() {
            //Ok(Either::Left(ready(Ok(form))))
            Ok(Box::pin(ready(Ok(form))))
        } else {
            let fut = self
                .files
                .into_iter()
                .map(|(k, f)| f.into_part().map(move |p| (k, p)))
                .collect::<FuturesUnordered<_>>()
                .map(Ok)
                .try_fold(form, |acc, (k, p)| async { Ok(acc.part(k, p?)) });

            //Ok(Either::Right(fut))
            Ok(Box::pin(fut))
        }
    }
}

struct PartSerializer {}

impl Serializer for PartSerializer {
    type Ok = (Part, Vec<(String, InputFile)>);
    type Error = Error;
    type SerializeSeq = InnerPartSerializer;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = PartSerializerStruct;
    type SerializeStructVariant = PartFromFile;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok((Part::text(v.to_string()), Vec::new()))
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok((Part::text(v.to_string()), Vec::new()))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok((Part::text(v.to_string()), Vec::new()))
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok((Part::text(v.to_owned()), Vec::new()))
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant_name: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok((Part::text(variant_name), Vec::new()))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let file: InputFile = InputFileUnserializer::NotMem.serialize_newtype_variant(
            name,
            variant_index,
            variant,
            value,
        )?;

        match file {
            f @ InputFile::Memory { .. } | f @ InputFile::File(_) => {
                let uuid = uuid::Uuid::new_v4().to_string();
                let part = Part::text(format!("attach://{}", uuid));
                Ok((part, vec![(uuid, f)]))
            }
            InputFile::FileId(s) | InputFile::Url(s) => Ok((Part::text(s), Vec::new())),
        }
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Self::SerializeSeq {
            array_json_parts: vec![],
            files: vec![],
        })
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let mut ser = serde_json::Serializer::new(Vec::new());
        ser.serialize_struct(name, len)?;
        Ok(PartSerializerStruct(
            ser, // TODO: capcity
            serde_json::ser::State::First,
            Vec::new(),
        ))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(PartFromFile {
            inner: InputFileUnserializer::memory().serialize_struct_variant(
                name,
                variant_index,
                variant,
                len,
            )?,
        })
    }
}

struct PartFromFile {
    inner: InputFileUnserializer,
}

impl SerializeStructVariant for PartFromFile {
    type Ok = (Part, Vec<(String, InputFile)>);
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.inner
            .serialize_field(key, value)
            .map_err(Error::InputFileUnserializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let file = self.inner.end()?;

        // TODO: to method
        match file {
            f @ InputFile::Memory { .. } | f @ InputFile::File(_) => {
                let uuid = uuid::Uuid::new_v4().to_string();
                let part = Part::text(format!("attach://{}", uuid));

                Ok((part, vec![(uuid, f)]))
            }
            InputFile::FileId(s) | InputFile::Url(s) => Ok((Part::text(s), vec![])),
        }
    }
}

struct InnerPartSerializer {
    array_json_parts: Vec<serde_json::Value>, // using value is such a workaround :|
    files: Vec<(String, InputFile)>,
}

impl SerializeSeq for InnerPartSerializer {
    type Ok = (Part, Vec<(String, InputFile)>);
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // NOTE: this is probably highly inefficient (especially for ::Memory),
        //       but at least it works
        let mut value = serde_json::to_value(value)?;
        let file: InputFile = serde_json::from_value(value["media"].take())?;

        match file {
            f @ InputFile::Memory { .. } | f @ InputFile::File(_) => {
                let uuid = uuid::Uuid::new_v4().to_string();
                value["media"] = serde_json::Value::String(format!("attach://{}", uuid));
                self.files.push((uuid, f));
            }
            InputFile::FileId(s) | InputFile::Url(s) => {
                value["media"] = serde_json::Value::String(s);
            }
        }

        self.array_json_parts.push(value);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let s = serde_json::to_string(&self.array_json_parts)?;
        Ok((Part::text(s), self.files))
    }
}

struct PartSerializerStruct(
    serde_json::Serializer<Vec<u8>>,
    serde_json::ser::State,
    Vec<(String, InputFile)>,
);

impl SerializeStruct for PartSerializerStruct {
    type Ok = (Part, Vec<(String, InputFile)>);
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let state = match self.1 {
            serde_json::ser::State::Empty => serde_json::ser::State::Empty,
            serde_json::ser::State::First => serde_json::ser::State::First,
            serde_json::ser::State::Rest => serde_json::ser::State::Rest,
        };
        let mut ser = serde_json::ser::Compound::Map {
            ser: &mut self.0,
            state,
        };

        // special case media (required for `edit_message_media` to work)
        if key == "media" {
            let file = value.serialize(InputFileUnserializer::NotMem)?;

            match file {
                f @ InputFile::Memory { .. } | f @ InputFile::File(_) => {
                    let uuid = uuid::Uuid::new_v4().to_string();
                    let attach = format!("attach://{}", uuid);

                    SerializeStruct::serialize_field(&mut ser, key, attach.as_str())?;
                    self.1 = get_state(ser);

                    self.2.push((uuid, f));
                }
                InputFile::FileId(s) | InputFile::Url(s) => {
                    SerializeStruct::serialize_field(&mut ser, key, &s)?;
                    self.1 = get_state(ser)
                }
            }
        } else {
            SerializeStruct::serialize_field(&mut ser, key, value)?;
            self.1 = get_state(ser);
        }

        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let state = match self.1 {
            serde_json::ser::State::Empty => serde_json::ser::State::Empty,
            serde_json::ser::State::First => serde_json::ser::State::First,
            serde_json::ser::State::Rest => serde_json::ser::State::Rest,
        };
        let ser = serde_json::ser::Compound::Map {
            ser: &mut self.0,
            state,
        };
        SerializeStruct::end(ser)?;

        let json = self.0.into_inner();
        Ok((Part::bytes(json), self.2))
    }
}

fn get_state(
    compound: serde_json::ser::Compound<Vec<u8>, serde_json::ser::CompactFormatter>,
) -> serde_json::ser::State {
    // Compound may have more variants under some serde_json features
    #[allow(unreachable_patterns)]
    match compound {
        serde_json::ser::Compound::Map { ser: _, state } => state,
        _ => unreachable!(),
    }
}
