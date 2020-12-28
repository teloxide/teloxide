use serde::de::DeserializeOwned;
use serde_json::{from_value, Value};

/// Similar to `Vec<T>` but if it fails to deserialize element, it just saves
/// `Err((value, err))`
#[derive(Debug, serde::Deserialize)]
#[serde(from = "Vec<serde_json::Value>")]
#[serde(bound = "T: DeserializeOwned")]
pub struct NonStrictVec<T>(pub Vec<Result<T, (serde_json::Value, serde_json::Error)>>);

impl<T: DeserializeOwned> From<Vec<serde_json::Value>> for NonStrictVec<T> {
    fn from(vec: Vec<Value>) -> Self {
        Self(
            vec.into_iter()
                .map(|val| from_value(val.clone()).map_err(|e| (val, e)))
                .collect(),
        )
    }
}

#[test]
fn test() {
    use crate::types::Update;

    let x: NonStrictVec<Update> = serde_json::from_str(
        r#"[{
         "update_id": 923808447,
         "message": {
          "message_id": 361678,
          "from": {
           "id": 218485655,
           "is_bot": false,
           "first_name": "вафель",
           "last_name": "🧇",
           "username": "WaffleLapkin",
           "language_code": "en"
          },
          "chat": {
           "id": 218485655,
           "first_name": "вафель",
           "last_name": "🧇",
           "username": "WaffleLapkin",
           "type": "private"
          },
          "date": 1595860067,
          "text": "s"
         }
        }]"#,
    )
    .unwrap();

    assert!(x.0.first().unwrap().is_ok())
}
