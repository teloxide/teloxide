use serde_json::{json, Value};
use teloxide_core::types::{
    ExternalReplyInfo, ExternalReplyInfoKind, InputMessageContent, Message, Update, UpdateKind,
    UserId, WidgetChatType, WidgetQuery, WidgetQueryType,
};

fn user() -> Value {
    json!({"id":7,"is_bot":false,"first_name":"Alice"})
}
fn query() -> Value {
    json!({"id":"q1","from":user(),"chat_instance":"chat-instance",
        "type":{"type":"data"},"data":"refresh",
        "chat":{"id":7,"type":"private","first_name":"Alice"},"message_id":42})
}

#[test]
fn ordinary_query_retains_actor_location_and_action() {
    let value = json!({"update_id":3,"widget_query":query()});
    let update: Update = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(update.from().unwrap().id, UserId(7));
    assert_eq!(update.chat().unwrap().id.0, 7);
    assert_eq!(update.mentioned_users().map(|u| u.id).collect::<Vec<_>>(), vec![UserId(7)]);
    let UpdateKind::WidgetQuery(ref q) = update.kind else { panic!("widget query lost") };
    assert_eq!(q.message_id.unwrap().0, 42);
    assert_eq!(q.data, "refresh");
    let roundtrip: Update = serde_json::from_str(&serde_json::to_string(&update).unwrap()).unwrap();
    assert_eq!(roundtrip, update);
}

#[test]
fn inline_query_has_no_fabricated_chat_or_message() {
    let mut q = query();
    q.as_object_mut().unwrap().remove("chat");
    q.as_object_mut().unwrap().remove("message_id");
    q["inline_message_id"] = json!("inline-id");
    q["chat_type"] = json!("sender");
    let update: Update = serde_json::from_value(json!({"update_id":4,"widget_query":q})).unwrap();
    assert!(update.chat().is_none());
    let UpdateKind::WidgetQuery(q) = update.kind else { panic!("inline widget query lost") };
    assert_eq!(q.chat_type, Some(WidgetChatType::Sender));
    assert!(q.message_id.is_none());
}

#[test]
fn forwarded_query_exposes_shared_identity_without_chat() {
    let value = json!({"update_id":5,"shared_widget_query":{"id":"q2","from":user(),
        "type":{"type":"data"},"data":"vote:1","widget_id":"shared-id"}});
    let update: Update = serde_json::from_value(value.clone()).unwrap();
    assert!(update.chat().is_none());
    assert_eq!(update.from().unwrap().id, UserId(7));
    assert!(matches!(update.kind, UpdateKind::SharedWidgetQuery(_)));
    assert_eq!(serde_json::to_value(update).unwrap(), value);
}

#[test]
fn wallet_outcomes_remain_typed_even_when_app_disables_them() {
    for kind in [
        json!({"type":"sign_data","wallet":{"address":"wallet","amount":"9007199254740993000"},"sign_data":"{}"}),
        json!({"type":"sent_transaction","wallet":{"address":"wallet","amount":"1","public_key":"key","state_init":"base64"},"msg_hash":"hash"}),
        json!({"type":"data","wallet_card":{"address":"wallet","amount":"1","stickers":[]}}),
    ] {
        let decoded: WidgetQueryType = serde_json::from_value(kind.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), kind);
    }
    let mut invalid = query();
    invalid.as_object_mut().unwrap().remove("type");
    assert!(serde_json::from_value::<WidgetQuery>(invalid).is_err());
}

#[test]
fn widget_message_keeps_caption_state_and_entities() {
    let value = json!({"message_id":42,"date":1794400000,
        "chat":{"id":7,"type":"private","first_name":"Alice"},
        "widget":{"html":"<p>Hello</p>","id":"shared-id","chat_data":"private","public_data":"public"},
        "caption":"Hello","caption_entities":[{"type":"bold","offset":0,"length":5}],
        "show_caption_above_media":true});
    let message: Message = serde_json::from_value(value).unwrap();
    assert_eq!(message.caption(), Some("Hello"));
    assert_eq!(message.caption_entities().unwrap().len(), 1);
    assert!(message.show_caption_above_media());
    assert!(!message.has_media_spoiler());
    assert_eq!(message.widget().unwrap().id.as_deref(), Some("shared-id"));
    let roundtrip: Message =
        serde_json::from_value(serde_json::to_value(&message).unwrap()).unwrap();
    assert_eq!(roundtrip, message);
}

#[test]
fn inline_content_distinguishes_new_and_reused_definitions() {
    let fresh: InputMessageContent =
        serde_json::from_value(json!({"html":"<p>New</p>","is_shared":true})).unwrap();
    assert!(matches!(fresh, InputMessageContent::Widget(_)));
    let shared: InputMessageContent =
        serde_json::from_value(json!({"widget_id":"existing","chat_data":"local"})).unwrap();
    assert!(matches!(shared, InputMessageContent::SharedWidget(_)));
}

#[test]
fn external_reply_keeps_widget() {
    let reply: ExternalReplyInfo = serde_json::from_value(json!({
        "origin":{"type":"user","date":1794400000,"sender_user":user()},
        "widget":{"html":"<p>Reply</p>"}
    }))
    .unwrap();
    assert!(matches!(reply.kind, ExternalReplyInfoKind::Widget(_)));
}
