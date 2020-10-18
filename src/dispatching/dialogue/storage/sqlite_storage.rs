use super::{serializer::Serializer, Storage};
use futures::future::BoxFuture;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection};
use sqlx::{ConnectOptions, Executor};
use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::Mutex;

// An error returned from [`SqliteStorage`].
#[derive(Debug, Error)]
pub enum SqliteStorageError<SE>
where
    SE: Debug + Display,
{
    #[error("dialogue serialization error: {0}")]
    SerdeError(SE),
    #[error("sqlite error: {0}")]
    SqliteError(#[from] sqlx::Error),
}

// TODO: make JSON serializer to be default
pub struct SqliteStorage<S> {
    conn: Mutex<SqliteConnection>,
    serializer: S,
}

impl<S> SqliteStorage<S> {
    pub async fn open(
        path: &str,
        serializer: S,
    ) -> Result<Arc<Self>, SqliteStorageError<Infallible>> {
        let mut conn =
            SqliteConnectOptions::new().filename(path).create_if_missing(true).c§onnect().await?;

        // TODO: think about a schema migration mechanism.
        conn.execute(
            r#"
CREATE TABLE IF NOT EXISTS teloxide_dialogues (
    chat_id BIGINT PRIMARY KEY,
    dialogue BLOB NOT NULL
);
        "#,
        )
        .await?;

        Ok(Arc::new(Self { conn: Mutex::new(conn), serializer }))
    }
}

impl<S, D> Storage<D> for SqliteStorage<S>
where
    S: Send + Sync + Serializer<D> + 'static,
    D: Send + Serialize + DeserializeOwned + 'static,
    <S as Serializer<D>>::Error: Debug + Display,
{
    type Error = SqliteStorageError<<S as Serializer<D>>::Error>;

    fn remove_dialogue(
        self: Arc<Self>,
        chat_id: i64,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move {
            self.conn
                .lock()
                .await
                .execute(
                    sqlx::query("DELETE FROM teloxide_dialogues WHERE chat_id = ?").bind(chat_id),
                )
                .await?;
            Ok(None)
        })
    }

    fn update_dialogue(
        self: Arc<Self>,
        chat_id: i64,
        dialogue: D,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move {
            let dialogue =
                self.serializer.serialize(&dialogue).map_err(SqliteStorageError::SerdeError)?;
            self.conn
                .lock()
                .await
                .execute(
                    sqlx::query(
                        r#"
INSERT INTO teloxide_dialogues VALUES (?, ?) WHERE chat_id = ?
ON CONFLICT(chat_id) DO UPDATE SET dialogue=excluded.dialogue
                    "#,
                    )
                    .bind(chat_id)
                    .bind(dialogue),
                )
                .await?;
            Ok(None)
        })
    }
}
