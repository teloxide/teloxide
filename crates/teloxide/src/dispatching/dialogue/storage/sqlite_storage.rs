use super::{serializer::Serializer, Storage};
use futures::future::BoxFuture;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{sqlite::SqlitePool, Executor};
use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    str,
    sync::Arc,
};
use teloxide_core::types::ChatId;
use thiserror::Error;

/// A persistent dialogue storage based on [SQLite](https://www.sqlite.org/).
pub struct SqliteStorage<S> {
    pool: SqlitePool,
    serializer: S,
}

/// An error returned from [`SqliteStorage`].
#[derive(Debug, Error)]
pub enum SqliteStorageError<SE>
where
    SE: Debug + Display,
{
    #[error("dialogue serialization error: {0}")]
    SerdeError(SE),

    #[error("sqlite error: {0}")]
    SqliteError(#[from] sqlx::Error),

    /// Returned from [`SqliteStorage::remove_dialogue`].
    #[error("row not found")]
    DialogueNotFound,
}

impl<S> SqliteStorage<S> {
    pub async fn open(
        path: &str,
        serializer: S,
    ) -> Result<Arc<Self>, SqliteStorageError<Infallible>> {
        let pool = SqlitePool::connect(format!("sqlite:{path}?mode=rwc").as_str()).await?;
        sqlx::query(
            "
CREATE TABLE IF NOT EXISTS teloxide_dialogues (
    chat_id BIGINT PRIMARY KEY,
    dialogue BLOB NOT NULL
);
        ",
        )
        .execute(&pool)
        .await?;

        Ok(Arc::new(Self { pool, serializer }))
    }
}

impl<S, D> Storage<D> for SqliteStorage<S>
where
    S: Send + Sync + Serializer<D> + 'static,
    D: Send + Serialize + DeserializeOwned + 'static,
    <S as Serializer<D>>::Error: Debug + Display,
{
    type Error = SqliteStorageError<<S as Serializer<D>>::Error>;

    /// Returns [`sqlx::Error::RowNotFound`] if a dialogue does not exist.
    fn remove_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
    ) -> BoxFuture<'static, Result<(), Self::Error>> {
        Box::pin(async move {
            let deleted_rows_count =
                sqlx::query("DELETE FROM teloxide_dialogues WHERE chat_id = ?")
                    .bind(chat_id)
                    .execute(&self.pool)
                    .await?
                    .rows_affected();

            if deleted_rows_count == 0 {
                return Err(SqliteStorageError::DialogueNotFound);
            }

            Ok(())
        })
    }

    fn update_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>> {
        Box::pin(async move {
            let d = self.serializer.serialize(&dialogue).map_err(SqliteStorageError::SerdeError)?;

            self.pool
                .acquire()
                .await?
                .execute(
                    sqlx::query(
                        "
            INSERT INTO teloxide_dialogues VALUES (?, ?)
            ON CONFLICT(chat_id) DO UPDATE SET dialogue=excluded.dialogue
                                ",
                    )
                    .bind(chat_id)
                    .bind(d),
                )
                .await?;
            Ok(())
        })
    }

    fn get_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move {
            get_dialogue(&self.pool, chat_id)
                .await?
                .map(|d| self.serializer.deserialize(&d).map_err(SqliteStorageError::SerdeError))
                .transpose()
        })
    }
}

async fn get_dialogue(
    pool: &SqlitePool,
    ChatId(chat_id): ChatId,
) -> Result<Option<Vec<u8>>, sqlx::Error> {
    #[derive(sqlx::FromRow)]
    struct DialogueDbRow {
        dialogue: Vec<u8>,
    }

    let bytes = sqlx::query_as::<_, DialogueDbRow>(
        "SELECT dialogue FROM teloxide_dialogues WHERE chat_id = ?",
    )
    .bind(chat_id)
    .fetch_optional(pool)
    .await?
    .map(|r| r.dialogue);

    Ok(bytes)
}
