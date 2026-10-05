use teloxide_core::{
    payloads::SendRichMessage,
    requests::MultipartPayload,
    types::{
        ChatId, CommunityChatJoined, EphemeralMessageParameters, InlineKeyboardButton,
        InlineKeyboardMarkup, InputFile, InputMediaDocument, InputRichBlock, InputRichBlockButtons,
        InputRichBlockDocument, InputRichMessage, KeyboardMarkup, RichBlock, RichBlockKind,
        RichMessage, RichMessageButton, RichText, Update, UpdateKind, UserId,
    },
};

#[test]
fn rich_message_10_3_blocks_and_buttons_round_trip() {
    let rich: RichMessage = serde_json::from_value(serde_json::json!({
        "blocks": [
            {
                "type": "buttons",
                "buttons": [
                    {"text": "Open", "callback_data": "open"},
                    {"text": "Unavailable", "disabled": {}}
                ],
                "align": "center"
            },
            {
                "type": "expandable_blockquote",
                "text": "Details",
                "credit": "Source"
            },
            {
                "type": "table",
                "cells": [],
                "is_compact": true
            },
            {
                "type": "document",
                "document": {
                    "file_id": "file-id",
                    "file_unique_id": "unique-id",
                    "file_size": 1
                }
            }
        ],
        "is_rtl": false
    }))
    .unwrap();

    assert!(matches!(
        &rich.blocks[0],
        RichBlock::Known(kind) if matches!(kind.as_ref(), RichBlockKind::Buttons(_))
    ));
    assert!(matches!(
        &rich.blocks[1],
        RichBlock::Known(kind) if matches!(kind.as_ref(), RichBlockKind::ExpandableBlockquote(_))
    ));
    assert!(matches!(
        &rich.blocks[2],
        RichBlock::Known(kind) if matches!(kind.as_ref(), RichBlockKind::Table(_))
    ));
    assert!(matches!(
        &rich.blocks[3],
        RichBlock::Known(kind) if matches!(kind.as_ref(), RichBlockKind::Document(_))
    ));

    let encoded = serde_json::to_value(&rich).unwrap();
    assert_eq!(encoded["blocks"][0]["buttons"][0]["callback_data"], "open");
    assert_eq!(encoded["blocks"][0]["buttons"][1]["disabled"], serde_json::json!({}));
    assert_eq!(encoded["blocks"][2]["is_compact"], true);

    let button_text: RichText = serde_json::from_value(serde_json::json!({
        "type": "button",
        "button": {"text": "Copy", "copy_text": {"text": "value"}}
    }))
    .unwrap();
    let button_text = serde_json::to_value(button_text).unwrap();
    assert_eq!(button_text["type"], "button");
    assert_eq!(button_text["button"]["copy_text"]["text"], "value");
}

#[test]
fn rich_document_collects_uploaded_files() {
    let rich = InputRichMessage::blocks([InputRichBlock::Document(InputRichBlockDocument {
        document: InputMediaDocument::new(InputFile::memory(vec![1, 2, 3])),
        caption: None,
    })]);

    let payload = SendRichMessage::new(ChatId(1), rich);
    let mut copied = 0;
    payload.copy_files(&mut |_| copied += 1);
    assert_eq!(copied, 1);
}

#[test]
fn disabled_and_force_reply_markup_serializes() {
    assert_eq!(
        serde_json::to_value(
            InlineKeyboardButton::callback("Open", "open")
                .style("primary")
                .icon_custom_emoji_id(teloxide_core::types::CustomEmojiId("emoji".into()))
        )
        .unwrap(),
        serde_json::json!({
            "text": "Open",
            "callback_data": "open",
            "style": "primary",
            "icon_custom_emoji_id": "emoji"
        })
    );
    assert_eq!(
        serde_json::to_value(InlineKeyboardButton::disabled("Unavailable")).unwrap(),
        serde_json::json!({"text": "Unavailable", "disabled": {}})
    );
    assert_eq!(
        serde_json::to_value(InlineKeyboardMarkup::default().force_reply()).unwrap(),
        serde_json::json!({"inline_keyboard": [], "force_reply": true})
    );
    assert_eq!(
        serde_json::to_value(KeyboardMarkup::default().force_reply()).unwrap()["force_reply"],
        true
    );
}

#[test]
fn rich_button_primitives_expose_native_style_and_alignment() {
    let buttons = InputRichBlockButtons::new([
        RichMessageButton::callback("Open", "open").style("primary"),
        RichMessageButton::disabled("Unavailable"),
    ])
    .align("center");
    let message = InputRichMessage::blocks([InputRichBlock::Buttons(buttons)]);

    assert_eq!(
        serde_json::to_value(message).unwrap(),
        serde_json::json!({
            "blocks": [{
                "type": "buttons",
                "buttons": [
                    {"text": "Open", "callback_data": "open", "style": "primary"},
                    {"text": "Unavailable", "disabled": {}}
                ],
                "align": "center"
            }]
        })
    );
}

#[test]
fn ephemeral_parameters_and_generation_stop_update_round_trip() {
    let parameters = EphemeralMessageParameters::new(UserId(7))
        .callback_query_id("query")
        .replace_callback_query_message(true);
    assert_eq!(
        serde_json::to_value(parameters).unwrap(),
        serde_json::json!({
            "receiver_user_id": 7,
            "callback_query_id": "query",
            "replace_callback_query_message": true
        })
    );

    let update: Update = serde_json::from_value(serde_json::json!({
        "update_id": 1,
        "stopped_message_generation": {
            "chat": {"id": 7, "type": "private", "first_name": "User"},
            "draft_id": 42
        }
    }))
    .unwrap();
    let UpdateKind::StoppedMessageGeneration(stopped) = update.kind else {
        panic!("expected stopped message generation update");
    };
    assert_eq!(stopped.chat.id.0, 7);
    assert_eq!(stopped.draft_id, 42);
}

#[test]
fn community_chat_joined_service_message_is_typed() {
    let message: teloxide_core::types::Message = serde_json::from_value(serde_json::json!({
        "message_id": 1,
        "date": 0,
        "chat": {"id": -100, "type": "supergroup", "title": "group"},
        "community_chat_joined": {"community": {"id": 42, "name": "Example"}}
    }))
    .unwrap();

    let joined: &CommunityChatJoined = message.community_chat_joined().unwrap();
    assert_eq!(joined.community.id, 42);
}
