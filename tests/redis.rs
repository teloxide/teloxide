use std::{
    fmt::{Debug, Display},
    future::Future,
    sync::Arc,
};
use teloxide::dispatching::dialogue::{
    serializer::{Bincode, CBOR, JSON},
    RedisStorage, Serializer, Storage,
};

#[tokio::test]
async fn test_redis_json() {
    let storage = Arc::new(
        RedisStorage::open("redis://127.0.0.1:7777", JSON).await.unwrap(),
    );
    test_redis(storage).await;
}

#[tokio::test]
async fn test_redis_bincode() {
    let storage = Arc::new(
        RedisStorage::open("redis://127.0.0.1:7778", Bincode).await.unwrap(),
    );
    test_redis(storage).await;
}

#[tokio::test]
async fn test_redis_cbor() {
    let storage = Arc::new(
        RedisStorage::open("redis://127.0.0.1:7779", CBOR).await.unwrap(),
    );
    test_redis(storage).await;
}

type Dialogue = String;

async fn test_redis<S>(storage: Arc<RedisStorage<S>>)
where
    S: Send + Sync + Serializer<Dialogue> + 'static,
    <S as Serializer<Dialogue>>::Error: Debug + Display,
{
    check_dialogue(
        None,
        Arc::clone(&storage).update_dialogue(1, "ABC".to_owned()),
    )
    .await;
    check_dialogue(
        None,
        Arc::clone(&storage).update_dialogue(11, "DEF".to_owned()),
    )
    .await;
    check_dialogue(
        None,
        Arc::clone(&storage).update_dialogue(256, "GHI".to_owned()),
    )
    .await;

    // 1 - ABC, 11 - DEF, 256 - GHI

    check_dialogue(
        "ABC",
        Arc::clone(&storage).update_dialogue(1, "JKL".to_owned()),
    )
    .await;
    check_dialogue(
        "GHI",
        Arc::clone(&storage).update_dialogue(256, "MNO".to_owned()),
    )
    .await;

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
