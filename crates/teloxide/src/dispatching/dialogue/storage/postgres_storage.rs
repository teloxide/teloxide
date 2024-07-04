use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    str,
    sync::Arc,
};

use futures::future::BoxFuture;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use teloxide_core::types::ChatId;
use thiserror::Error;

use super::{serializer::Serializer, Storage};

/// An error returned from [`PostgresStorage`].
#[derive(Debug, Error)]
pub enum PostgresStorageError<SE>
where
    SE: Debug + Display,
{
    #[error("dialogue serialization error: {0}")]
    SerdeError(SE),

    #[error("postgres error: {0}")]
    PostgresError(#[from] sqlx::Error),

    // TODO maybe add chat_id for the sake of completeness?
    #[error("row not found")]
    DialogueNotFound,
}

/// A persistent dialogue storage based on [PostgreSQL](https://www.postgresql.org/)
pub struct PostgresStorage<S> {
    pool: PgPool,
    serializer: S,
}

impl<S> PostgresStorage<S> {
    /// Opens a connection pool to the [Postgres](https://www.postgresql.org/) database and creates the table
    /// for storing dialogues.
    ///
    /// Parameters:
    /// - database_url: full url to the postgres database, for example
    ///   `"postgres://postgres:password@localhost/test")`
    /// - max_connections: number of connections in creating connection pool. Be
    ///   mindful of the connection limits for your database, each connection
    ///   established with the Postgres creates a new process on the server side
    /// - serializer: what [`Serializer`] will be used to encode the dialogue
    ///   data. Available ones are: [`Json`], [`Bincode`], [`Cbor`]
    ///
    /// [`Json`]: crate::dispatching::dialogue::serializer::Json
    /// [`Bincode`]: crate::dispatching::dialogue::serializer::Bincode
    /// [`Cbor`]: crate::dispatching::dialogue::serializer::Cbor
    pub async fn open(
        database_url: &str,
        max_connections: u32,
        serializer: S,
    ) -> Result<Arc<Self>, PostgresStorageError<Infallible>> {
        let pool =
            PgPoolOptions::new().max_connections(max_connections).connect(database_url).await?;
        sqlx::query(include_str!("postgres_storage/queries/create_teloxide_dialogues.sql"))
            .execute(&pool)
            .await?;

        Ok(Arc::new(Self { pool, serializer }))
    }

    async fn get_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
    ) -> Result<Option<Vec<u8>>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct DialogueDbRow {
            dialogue: Vec<u8>,
        }

        let bytes = sqlx::query_as::<_, DialogueDbRow>(include_str!(
            "postgres_storage/queries/get_dialogue.sql"
        ))
        .bind(chat_id)
        .fetch_optional(&self.pool)
        .await?
        .map(|r| r.dialogue);

        Ok(bytes)
    }
}

// FIXME: these methods' bodies are almostly the same as SqliteStorage ones
// (except actual queries) Maybe combine them somehow?

impl<S, D> Storage<D> for PostgresStorage<S>
where
    S: Send + Sync + Serializer<D> + 'static,
    D: Send + Serialize + DeserializeOwned + 'static,
    <S as Serializer<D>>::Error: Debug + Display,
{
    type Error = PostgresStorageError<<S as Serializer<D>>::Error>;

    fn remove_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            let deleted_rows_count =
                sqlx::query(include_str!("postgres_storage/queries/remove_dialogue.sql"))
                    .bind(chat_id)
                    .execute(&self.pool)
                    .await?
                    .rows_affected();

            if deleted_rows_count == 0 {
                return Err(PostgresStorageError::DialogueNotFound);
            }

            Ok(())
        })
    }

    fn update_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            let d =
                self.serializer.serialize(&dialogue).map_err(PostgresStorageError::SerdeError)?;
            sqlx::query(include_str!("postgres_storage/queries/update_dialogue.sql"))
                .bind(chat_id)
                .bind(d)
                .execute(&self.pool)
                .await?;
            Ok(())
        })
    }

    fn get_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move {
            self.clone()
                .get_dialogue(chat_id)
                .await?
                .map(|d| self.serializer.deserialize(&d).map_err(PostgresStorageError::SerdeError))
                .transpose()
        })
    }
}
