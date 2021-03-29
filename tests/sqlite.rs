use std::{
    fmt::{Debug, Display},
    sync::Arc,
};
use teloxide::dispatching::dialogue::{Serializer, SqliteStorage, Storage};

#[tokio::test(flavor = "multi_thread")]
async fn test_sqlite_json() {
    let storage =
        SqliteStorage::open("./test_db1.sqlite", teloxide::dispatching::dialogue::serializer::Json)
            .await
            .unwrap();
    test_sqlite(storage).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sqlite_bincode() {
    let storage = SqliteStorage::open(
        "./test_db2.sqlite",
        teloxide::dispatching::dialogue::serializer::Bincode,
    )
    .await
    .unwrap();
    test_sqlite(storage).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sqlite_cbor() {
    let storage =
        SqliteStorage::open("./test_db3.sqlite", teloxide::dispatching::dialogue::serializer::Cbor)
            .await
            .unwrap();
    test_sqlite(storage).await;
}

type Dialogue = String;

macro_rules! test_dialogues {
    ($storage:expr, $_0:expr, $_1:expr, $_2:expr) => {
        assert_eq!(Arc::clone(&$storage).get_dialogue(1).await.unwrap(), $_0);
        assert_eq!(Arc::clone(&$storage).get_dialogue(11).await.unwrap(), $_1);
        assert_eq!(Arc::clone(&$storage).get_dialogue(256).await.unwrap(), $_2);
    };
}

async fn test_sqlite<S>(storage: Arc<SqliteStorage<S>>)
where
    S: Send + Sync + Serializer<Dialogue> + 'static,
    <S as Serializer<Dialogue>>::Error: Debug + Display,
{
    test_dialogues!(storage, None, None, None);

    Arc::clone(&storage).update_dialogue(1, "ABC".to_owned()).await.unwrap();
    Arc::clone(&storage).update_dialogue(11, "DEF".to_owned()).await.unwrap();
    Arc::clone(&storage).update_dialogue(256, "GHI".to_owned()).await.unwrap();

    test_dialogues!(
        storage,
        Some("ABC".to_owned()),
        Some("DEF".to_owned()),
        Some("GHI".to_owned())
    );

    Arc::clone(&storage).remove_dialogue(1).await.unwrap();
    Arc::clone(&storage).remove_dialogue(11).await.unwrap();
    Arc::clone(&storage).remove_dialogue(256).await.unwrap();

    test_dialogues!(storage, None, None, None);
}
