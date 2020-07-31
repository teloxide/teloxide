use super::{serializer::Serializer, Storage};
use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    sync::Arc,
};
use rusqlite::{params, Connection, Error, Result};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use tokio::sync::Mutex;

pub enum SqliteStorageLocation {
    InMemory,
    Path(String),
}

// An error returned from [`SqliteStorage`].
#[derive(Debug, Error)]
pub enum SqliteStorageError<SE>
where
    SE: Debug + Display,
{
    #[error("parsing/serializing error: {0}")]
    SerdeError(SE),
    #[error("error from Sqlite: {0}")]
    SqliteError(#[from] Error),
}

pub struct SqliteStorage<S> {
    conn: Mutex<Connection>,
    serializer: S,
}

impl <S> SqliteStorage<S> {
    pub async fn open(
        path: SqliteStorageLocation,
        serializer: S,
    ) -> Result<Arc<Self>, SqliteStorageError<Infallible>>{
        let url = match path {
            SqliteStorageLocation::InMemory => String::from("sqlite::memory:"),
            SqliteStorageLocation::Path(p) => p,
        };
        Ok(Arc::new(Self {
            conn: Mutex::new(Connection::open(&url[..])?),
            serializer,
        }))
    }
}
