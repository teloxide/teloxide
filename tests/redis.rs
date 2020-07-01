use std::{
    fmt::{Debug, Display},
    future::Future,
    sync::Arc,
};
use teloxide::{
    dispatching::dialogue::{
        serializer::{Bincode, CBOR, JSON},
        RedisStorage, Serializer, Storage,
    },
    prelude::*,
};

#[tokio::test]
async fn test_redis_json() {
    test_redis(JSON).await;
}

#[tokio::test]
async fn test_redis_bincode() {
    test_redis(Bincode).await;
}

#[tokio::test]
async fn test_redis_cbor() {
    test_redis(CBOR).await;
}

type Dialogue = String;

async fn test_redis<S>(serializer: S)
where
    S: Send + Sync + Serializer<Dialogue> + 'static,
    <S as Serializer<Dialogue>>::Error: Debug + Display,
{
    let storage = Arc::new(
        RedisStorage::open("redis://127.0.0.1:7777", serializer).await.unwrap(),
    );

    check_dialogue(
        None,
        Arc::clone(&storage).update_dialogue(11, "ABC".to_owned()),
    );
    check_dialogue(
        None,
        Arc::clone(&storage).update_dialogue(256, "DEF".to_owned()),
    );
    check_dialogue(
        None,
        Arc::clone(&storage).update_dialogue(11, "GHI".to_owned()),
    );

    check_dialogue(
        "ABC",
        Arc::clone(&storage).update_dialogue(1, "JKL".to_owned()),
    );
    check_dialogue(
        "GHI",
        Arc::clone(&storage).update_dialogue(11, "MNO".to_owned()),
    );

    check_dialogue("JKL", Arc::clone(&storage).remove_dialogue(1));
    check_dialogue("DEF", Arc::clone(&storage).remove_dialogue(256));
    check_dialogue("MNO", Arc::clone(&storage).remove_dialogue(11));
}

async fn check_dialogue<E>(
    expected: impl Into<Option<&str>>,
    actual: impl Future<Output = Result<Option<Dialogue>, E>>,
) where
    E: Debug,
{
    assert_eq!(expected.into().map(ToOwned::to_owned), actual.await.unwrap())
}
