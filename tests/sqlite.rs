use std::{
    fmt::{Debug, Display},
    future::Future,
    sync::Arc,
};
use teloxide::dispatching::dialogue::{Serializer, SqliteStorage, Storage};

#[tokio::test(threaded_scheduler)]
async fn test_sqlite_json() {
    let storage =
        SqliteStorage::open("./test_db1.sqlite", teloxide::dispatching::dialogue::serializer::JSON)
            .await
            .unwrap();
    test_sqlite(storage).await;
}

#[tokio::test(threaded_scheduler)]
async fn test_sqlite_bincode() {
    let storage = SqliteStorage::open(
        "./test_db2.sqlite",
        teloxide::dispatching::dialogue::serializer::Bincode,
    )
    .await
    .unwrap();
    test_sqlite(storage).await;
}

#[tokio::test(threaded_scheduler)]
async fn test_sqlite_cbor() {
    let storage =
        SqliteStorage::open("./test_db3.sqlite", teloxide::dispatching::dialogue::serializer::CBOR)
            .await
            .unwrap();
    test_sqlite(storage).await;
}

type Dialogue = String;

async fn test_sqlite<S>(storage: Arc<SqliteStorage<S>>)
where
    S: Send + Sync + Serializer<Dialogue> + 'static,
    <S as Serializer<Dialogue>>::Error: Debug + Display,
{
    check_dialogue(None, Arc::clone(&storage).update_dialogue(1, "ABC".to_owned())).await;
    check_dialogue(None, Arc::clone(&storage).update_dialogue(11, "DEF".to_owned())).await;
    check_dialogue(None, Arc::clone(&storage).update_dialogue(256, "GHI".to_owned())).await;

    // 1 - ABC, 11 - DEF, 256 - GHI

    check_dialogue("ABC", Arc::clone(&storage).update_dialogue(1, "JKL".to_owned())).await;
    check_dialogue("GHI", Arc::clone(&storage).update_dialogue(256, "MNO".to_owned())).await;

    // 1 - GKL, 11 - DEF, 256 - MNO

    check_dialogue("JKL", Arc::clone(&storage).remove_dialogue(1)).await;
    check_dialogue("DEF", Arc::clone(&storage).remove_dialogue(11)).await;
    check_dialogue("MNO", Arc::clone(&storage).remove_dialogue(256)).await;
}

async fn check_dialogue<E>(
    expected: impl Into<Option<&str>>,
    actual: impl Future<Output = Result<Option<Dialogue>, E>>,
) where
    E: Debug,
{
    assert_eq!(expected.into().map(ToOwned::to_owned), actual.await.unwrap())
}
