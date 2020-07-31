// use super::{serializer::Serializer, Storage};
// use futures::future::BoxFuture;
use std::{
    convert::Infallible,
    fmt::{Debug, Display},
};
use sqlx::sqlite::SqlitePool;
// use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
// use tokio::task::block_in_place;

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
    SqliteError(Box<dyn std::error::Error>),
}

pub struct SqliteStorage<S> {
    conn: SqlitePool,
    serializer: S,
}

impl <S> SqliteStorage<S> {
    pub async fn open(
        path: SqliteStorageLocation,
        serializer: S,
    ) -> Result<Self, SqliteStorageError<Infallible>>{
        let url = match path {
            SqliteStorageLocation::InMemory => String::from("sqlite::memory:"),
            SqliteStorageLocation::Path(p) => p,
        };
        Ok(Self {
            conn: SqlitePool::connect(&url[..]).await
                .expect("Impossible sqlite error"),
            serializer,
        })
    }
}

// impl<S, D> Storage<D> for SqliteStorage<S>
// where
// S: Send + Sync + Serializer<D> + 'static,
// D: Send + Serialize + DeserializeOwned + 'static,
// <S as Serializer<D>>::Error: Debug + Display,
// {
//     type Error = SqliteStorageError<<S as Serializer<D>>::Error>;

//     fn remove_dialogue(
//         self: Arc<Self>,
//         chat_id: i64,
//     ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
//         Box::pin(async move {
//             todo!()
//         });
//     }

//     fn update_dialogue(
//         self: Arc<Self>,
//         chat_id: i64,
//         dialogue: D
//     ) { todo!() }
// }