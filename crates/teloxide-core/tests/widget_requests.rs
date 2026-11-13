use serde_json::json;
use teloxide_core::{
    payloads::{
        AnswerWidgetQuery, AnswerWidgetQuerySetters, EditMessageWidgetData,
        EditMessageWidgetDataInline, EditMessageWidgetDataInlineSetters,
        EditMessageWidgetDataSetters, EditSharedWidgetData, EditSharedWidgetDataSetters,
        SendSharedWidget, SendSharedWidgetSetters, SendWidget, SendWidgetSetters,
    },
    requests::Payload,
    types::{ChatId, MessageId, True},
};

#[test]
fn send_widget_preserves_html_and_all_state_flags() {
    let request = SendWidget::new(ChatId(17), "<button>Go</button>")
        .chat_data("{\"r\":1}")
        .public_data("public")
        .allow_queries(true)
        .is_shared(true)
        .caption("Useful caption");
    assert_eq!(SendWidget::NAME, "SendWidget");
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "chat_id":17,"html":"<button>Go</button>","chat_data":"{\"r\":1}",
            "public_data":"public","allow_queries":true,"is_shared":true,
            "caption":"Useful caption"
        })
    );
}

#[test]
fn shared_send_reuses_identity_without_recreating_definition() {
    let request =
        SendSharedWidget::new(String::from("@target"), "shared-id").chat_data("private-to-message");
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "chat_id":"@target","widget_id":"shared-id","chat_data":"private-to-message"
        })
    );
}

#[test]
fn omitted_edit_state_is_distinct_from_explicit_clear() {
    let request = EditMessageWidgetData::new(ChatId(17), MessageId(9)).chat_data("");
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "chat_id":17,"message_id":9,"chat_data":""
        })
    );
    let inline = EditMessageWidgetDataInline::new("inline-id").public_data("next");
    assert_eq!(
        serde_json::to_value(inline).unwrap(),
        json!({
            "inline_message_id":"inline-id","public_data":"next"
        })
    );
}

#[test]
fn shared_edit_includes_cache_policy_only_when_selected() {
    let request = EditSharedWidgetData::new("shared-id", "next").cache_time(30);
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "widget_id":"shared-id","public_data":"next","cache_time":30
        })
    );
}

#[test]
fn query_answer_is_separate_from_state_mutation() {
    let request = AnswerWidgetQuery::new("query-id").data("accepted");
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "widget_query_id":"query-id","data":"accepted"
        })
    );
    fn boolean_response<P: Payload<Output = True>>() {}
    boolean_response::<AnswerWidgetQuery>();
    boolean_response::<EditMessageWidgetData>();
    boolean_response::<EditSharedWidgetData>();
}

#[cfg(feature = "throttle")]
#[tokio::test]
async fn chat_widget_edits_are_throttled_and_inline_targets_are_explicit() {
    use teloxide_core::{
        adaptors::throttle::{Limits, ThrottlingRequest},
        requests::{Requester, RequesterExt},
        Bot,
    };
    let bot = Bot::new("fixture-token").throttle(Limits::default());
    let _: ThrottlingRequest<_> = bot.edit_message_widget_data(ChatId(17), MessageId(9));
    let request: ThrottlingRequest<_> = bot.edit_message_widget_data_inline("inline-id");
    use teloxide_core::requests::HasPayload;
    assert_eq!(request.payload_ref().inline_message_id, "inline-id");
}
