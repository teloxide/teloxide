use std::{any::TypeId, time::Duration};

use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Client, Response,
};
use serde::de::DeserializeOwned;

use crate::{net::TelegramResponse, requests::ResponseResult, RequestError};

const DELAY_ON_SERVER_ERROR: Duration = Duration::from_secs(10);

pub async fn request_multipart<T>(
    client: &Client,
    token: &str,
    api_url: reqwest::Url,
    method_name: &str,
    params: reqwest::multipart::Form,
    _timeout_hint: Option<Duration>,
) -> ResponseResult<T>
where
    T: DeserializeOwned + 'static,
{
    // Workaround for [#460]
    //
    // Telegram has some methods that return either `Message` or `True` depending on
    // the used arguments we model this as `...` and `..._inline` pairs of methods.
    //
    // Currently inline versions have wrong Payload::NAME (ie with the "Inline"
    // suffix). This removes the suffix allowing to call the right telegram method.
    // Note that currently there are no normal telegram methods ending in "Inline",
    // so this is fine.
    //
    // [#460]: https://github.com/teloxide/teloxide/issues/460
    let method_name = method_name.trim_end_matches("Inline");

    let request = client
        .post(crate::net::method_url(api_url, token, method_name))
        .multipart(params)
        .build()?;

    // FIXME: uncomment this, when reqwest starts setting default timeout early
    // if let Some(timeout) = timeout_hint {
    //     *request.timeout_mut().get_or_insert(Duration::ZERO) += timeout;
    // }

    let response = client.execute(request).await?;

    process_response(response).await
}

pub async fn request_json<T>(
    client: &Client,
    token: &str,
    api_url: reqwest::Url,
    method_name: &str,
    params: Vec<u8>,
    _timeout_hint: Option<Duration>,
) -> ResponseResult<T>
where
    T: DeserializeOwned + 'static,
{
    // Workaround for [#460]
    //
    // Telegram has some methods that return either `Message` or `True` depending on
    // the used arguments we model this as `...` and `..._inline` pairs of methods.
    //
    // Currently inline versions have wrong Payload::NAME (ie with the "Inline"
    // suffix). This removes the suffix allowing to call the right telegram method.
    // Note that currently there are no normal telegram methods ending in "Inline",
    // so this is fine.
    //
    // [#460]: https://github.com/teloxide/teloxide/issues/460
    let method_name = method_name.trim_end_matches("Inline");

    let request = client
        .post(crate::net::method_url(api_url, token, method_name))
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(params)
        .build()?;

    // FIXME: uncomment this, when reqwest starts setting default timeout early
    // if let Some(timeout) = timeout_hint {
    //     *request.timeout_mut().get_or_insert(Duration::ZERO) += timeout;
    // }

    let response = client.execute(request).await?;

    process_response(response).await
}

async fn process_response<T>(response: Response) -> ResponseResult<T>
where
    T: DeserializeOwned + 'static,
{
    if response.status().is_server_error() {
        tokio::time::sleep(DELAY_ON_SERVER_ERROR).await;
    }

    let text = response.text().await?;

    deserialize_response(text)
}

fn deserialize_response<T>(text: String) -> Result<T, RequestError>
where
    T: DeserializeOwned + 'static,
{
    serde_json::from_str::<TelegramResponse<T>>(&text)
        .map(|mut response| {
            use crate::types::{Update, UpdateKind};
            use std::{any::Any, iter::zip};

            // HACK: Fill-in error information into `UpdateKind::Error`.
            //
            //       Why? Well, we need `Update` deserialization to be reliable,
            //       even if Telegram breaks something in their Bot API, we want
            //       1. Deserialization to """succeed"""
            //       2. Get the `update.id`
            //
            //       Both of these points are required for `get_updates(...) -> Vec<Update>`
            //       to behave well after Telegram introduces updates that we can't parse.
            //       (1.) makes it so only some of the updates in a butch need to be skipped
            //       (otherwise serde'll stop on the first error). (2.) allows us to issue
            //       the next `get_updates` call with the right offset, even if the last
            //       update in the batch didn't deserialize well.
            //
            //       serde's interface doesn't allows us to implement `Deserialize` in such
            //       a way, that we could keep the data we couldn't parse, so our
            //       `Deserialize` impl for `UpdateKind` just returns
            //       `UpdateKind::Error(/* some empty-ish value */)`. Here, through some
            //       terrible hacks and downcasting, we fill-in the data we couldn't parse
            //       so that our users can make actionable bug reports.
            //
            //       We specifically handle `Vec<Update>` here, because that's the return
            //       type of the only method that returns updates.
            if TypeId::of::<T>() == TypeId::of::<Vec<Update>>() {
                if let TelegramResponse::Ok { response, .. } = &mut response {
                    if let Some(updates) =
                        (response as &mut T as &mut dyn Any).downcast_mut::<Vec<Update>>()
                    {
                        if updates.iter().any(|u| matches!(u.kind, UpdateKind::Error(_))) {
                            let re_parsed = serde_json::from_str(&text);

                            if let Ok(TelegramResponse::Ok { response: values, .. }) = re_parsed {
                                for (update, value) in zip::<_, Vec<_>>(updates, values) {
                                    if let UpdateKind::Error(dest) = &mut update.kind {
                                        *dest = value;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            response
        })
        .map_err(|source| RequestError::InvalidJson { source, raw: text.into() })?
        .into()
}

#[cfg(test)]
mod tests {
    use cool_asserts::assert_matches;

    use crate::{
        net::request::deserialize_response,
        types::{ChatId, Seconds, True, Update, UpdateId, UpdateKind},
        ApiError, RequestError,
    };

    #[test]
    fn smoke_ok() {
        let json = r#"{"ok":true,"result":true}"#.to_owned();

        let res = deserialize_response::<True>(json);
        assert_matches!(res, Ok(True));
    }

    #[test]
    fn smoke_err() {
        let json =
            r#"{"ok":false,"description":"Forbidden: bot was blocked by the user"}"#.to_owned();

        let res = deserialize_response::<True>(json);
        assert_matches!(res, Err(RequestError::Api(ApiError::BotBlocked)));
    }

    #[test]
    fn migrate() {
        let json = r#"{"ok":false,"description":"this string is ignored","parameters":{"migrate_to_chat_id":123456}}"#.to_owned();

        let res = deserialize_response::<True>(json);
        assert_matches!(res, Err(RequestError::MigrateToChatId(ChatId(123456))));
    }

    #[test]
    fn retry_after() {
        let json = r#"{"ok":false,"description":"this string is ignored","parameters":{"retry_after":123456}}"#.to_owned();

        let res = deserialize_response::<True>(json);
        assert_matches!(res, Err(RequestError::RetryAfter(duration)) if duration == Seconds::from_seconds(123456));
    }

    #[test]
    fn update_ok() {
        let json = r#"{
            "ok":true,
            "result":[
                {
                    "update_id":0,
                    "poll_answer":{
                        "poll_id":"POLL_ID",
                        "user": {"id":42,"is_bot":false,"first_name":"blah"},
                        "option_ids": []
                    }
                }
            ]
        }"#
        .to_owned();

        let res = deserialize_response::<Vec<Update>>(json).unwrap();
        assert_matches!(res, [Update { id: UpdateId(0), kind: UpdateKind::PollAnswer(_) }]);
    }

    /// Check that `get_updates` can work with malformed updates.
    #[test]
    fn update_err() {
        let json = r#"{
            "ok":true,
            "result":[
                {
                    "update_id":0,
                    "poll_answer":{
                        "poll_id":"POLL_ID",
                        "user": {"id":42,"is_bot":false,"first_name":"blah"},
                        "option_ids": []
                    }
                },
                {
                    "update_id":1,
                    "something unknown to us":17
                },
                {
                    "update_id":2,
                    "poll_answer":{
                        "poll_id":"POLL_ID",
                        "user": {"id":42,"is_bot":false,"first_name":"blah"},
                        "option_ids": [3, 4, 8]
                    }
                },
                {
                    "update_id":3,
                    "message":{"some fields are missing":true}
                }
            ]
        }"#
        .to_owned();

        let res = deserialize_response::<Vec<Update>>(json).unwrap();
        assert_matches!(
            res,
            [Update { id: UpdateId(0), kind: UpdateKind::PollAnswer(_) }, Update { id: UpdateId(1), kind: UpdateKind::Error(v) } if v.is_object(), Update { id: UpdateId(2), kind: UpdateKind::PollAnswer(_) }, Update { id: UpdateId(3), kind: UpdateKind::Error(v) } if v.is_object()]
        );
    }
}
