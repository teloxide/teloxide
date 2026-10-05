use teloxide_core::{
    payloads::EditMessageText,
    requests::{MultipartPayload, MultipartRequest},
    types::{
        AllowedUpdate, BotCommand, BotSubscriptionState, ChatFullInfo, ChatId, ChatJoinRequest,
        InputFile, InputMediaPhoto, InputRichMessage, InputRichMessageMedia,
        InputRichMessageMediaContent, MediaKind, Message, MessageId, MessageKind, Update,
        UpdateKind, UserId,
    },
    Bot,
};

#[test]
fn rich_message_response_preserves_content_and_ephemeral_identifiers() {
    let message: Message = serde_json::from_value(serde_json::json!({
        "message_id": 42,
        "date": 0,
        "chat": {"id": 1, "type": "private", "first_name": "receiver"},
        "receiver_user": {"id": 7, "is_bot": false, "first_name": "receiver"},
        "ephemeral_message_id": 99,
        "rich_message": {"blocks": [], "is_rtl": true}
    }))
    .unwrap();

    assert_eq!(message.ephemeral_message_id, Some(99));
    assert_eq!(message.receiver_user.as_ref().map(|user| user.id), Some(UserId(7)));

    let MessageKind::Common(common) = message.kind else {
        panic!("expected common rich message");
    };
    let MediaKind::RichMessage(rich) = common.media_kind else {
        panic!("expected rich message media kind");
    };
    assert!(rich.rich_message.blocks.is_empty());
    assert_eq!(rich.rich_message.is_rtl, Some(true));
}

#[test]
fn rich_only_edit_collects_local_media_and_uses_multipart() {
    fn assert_multipart(_: MultipartRequest<EditMessageText>) {}

    let rich = InputRichMessage::html(r#"<img src="tg://photo?id=cover">"#).media([
        InputRichMessageMedia::new(
            "cover",
            InputRichMessageMediaContent::Photo(InputMediaPhoto::new(InputFile::memory(vec![1]))),
        ),
    ]);
    let mut payload = EditMessageText::rich(ChatId(1), MessageId(2), rich.clone());

    let mut copied = 0;
    payload.copy_files(&mut |_| copied += 1);
    assert_eq!(copied, 1);

    let mut moved = 0;
    payload.move_files(&mut |_| moved += 1);
    assert_eq!(moved, 1);

    assert_multipart(Bot::new("token").edit_message_rich_text(ChatId(1), MessageId(2), rich));
}

#[test]
fn join_request_keeps_query_id_needed_by_query_methods() {
    let request: ChatJoinRequest = serde_json::from_value(serde_json::json!({
        "chat": {"id": -100, "type": "supergroup", "title": "group"},
        "query_id": "join-query",
        "from": {"id": 7, "is_bot": false, "first_name": "joiner"},
        "user_chat_id": 7,
        "date": 0
    }))
    .unwrap();

    assert_eq!(request.query_id.as_deref(), Some("join-query"));
}

#[test]
fn ordinary_command_omits_absent_ephemeral_flag() {
    assert_eq!(
        serde_json::to_value(BotCommand::new("help", "Help")).unwrap(),
        serde_json::json!({"command": "help", "description": "Help"})
    );
}

#[test]
fn ephemeral_command_flag_serializes() {
    let command = BotCommand::new("private", "Private reply").is_ephemeral(true);
    assert_eq!(
        serde_json::to_value(command).unwrap(),
        serde_json::json!({
            "command": "private",
            "description": "Private reply",
            "is_ephemeral": true,
        })
    );
}

#[test]
fn community_service_messages_and_chat_info_preserve_community() {
    let added: Message = serde_json::from_value(serde_json::json!({
        "message_id": 1,
        "date": 0,
        "chat": {"id": -100, "type": "supergroup", "title": "group"},
        "community_chat_added": {"community": {"id": 42, "name": "Example"}}
    }))
    .unwrap();
    assert_eq!(added.community_chat_added().unwrap().community.name, "Example");

    let removed: Message = serde_json::from_value(serde_json::json!({
        "message_id": 2,
        "date": 0,
        "chat": {"id": -100, "type": "supergroup", "title": "group"},
        "community_chat_removed": {}
    }))
    .unwrap();
    assert!(removed.community_chat_removed().is_some());

    let chat: ChatFullInfo = serde_json::from_value(serde_json::json!({
        "id": -100,
        "type": "supergroup",
        "title": "group",
        "accent_color_id": 0,
        "max_reaction_count": 1,
        "accepted_gift_types": {
            "unlimited_gifts": true,
            "limited_gifts": true,
            "unique_gifts": true,
            "premium_subscription": true,
            "gifts_from_channels": false
        },
        "community": {"id": 42, "name": "Example"}
    }))
    .unwrap();
    assert_eq!(chat.community.unwrap().id, 42);
}

#[test]
fn subscription_update_is_routed_and_allowed() {
    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "subscription": {
            "user": {"id": 7, "is_bot": false, "first_name": "subscriber"},
            "invoice_payload": "sub-42",
            "state": "active"
        }
    }))
    .unwrap();
    let UpdateKind::Subscription(subscription) = update.kind else {
        panic!("expected subscription update");
    };
    assert_eq!(subscription.state, BotSubscriptionState::Active);
    assert_eq!(subscription.user.id, UserId(7));
    assert_eq!(
        serde_json::to_value(AllowedUpdate::Subscription).unwrap(),
        serde_json::json!("subscription")
    );

    let future: Update = serde_json::from_value(serde_json::json!({
        "update_id": 2,
        "subscription": {
            "user": {"id": 8, "is_bot": false, "first_name": "subscriber"},
            "invoice_payload": "sub-43",
            "state": "paused"
        }
    }))
    .unwrap();
    let UpdateKind::Subscription(subscription) = future.kind else {
        panic!("expected subscription update");
    };
    assert_eq!(subscription.state, BotSubscriptionState::Unknown("paused".to_owned()));
}
