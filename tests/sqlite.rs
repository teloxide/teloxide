use teloxide::dispatching::dialogue::{
    serializer::{Bincode, CBOR, JSON},
    SqliteStorage, SqliteStorageLocation::InMemory
};

#[tokio::test]
async fn test_sqlite_json() {
    let _storage = SqliteStorage::open(InMemory, JSON).await.unwrap();
}

#[tokio::test]
async fn test_sqlite_cbor() {
    let _storage = SqliteStorage::open(InMemory, CBOR).await.unwrap();
}

#[tokio::test]
async fn test_sqlite_bincode() {
    let _storage = SqliteStorage::open(InMemory, Bincode).await.unwrap();
}