use super::{serializer::Serializer, Storage};
use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    sync::Arc,
};
use sqlx::{SqliteConnection, Connection, sqlite::SqliteError};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use tokio::{
    sync::Mutex,
    task::block_in_place,
};

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
    SqliteError(#[from] SqliteError),
}

pub struct SqliteStorage<S> {
    conn: Mutex<SqliteConnection>,
    serializer: S,
}

impl <S> SqliteStorage<S> {
    pub async fn open(
        path: SqliteStorageLocation,
        serializer: S,
    ) -> Result<Arc<Self>, Box<dyn std::error::Error>>{
        let url = match path {
            SqliteStorageLocation::InMemory => String::from("sqlite::memory:"),
            SqliteStorageLocation::Path(p) => p,
        };
        Ok(Arc::new(Self {
            conn: Mutex::new(SqliteConnection::connect(&url[..]).await?),
            serializer,
        }))
    }
}
