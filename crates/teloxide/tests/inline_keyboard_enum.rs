#[cfg(feature = "macros")]
use teloxide::utils::button::InlineButtons;

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_unit_variants() {
    use teloxide::types::InlineKeyboardButtonKind;

    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "A", row = 1)]
        A,
        #[button(text = "B", row = 2)]
        B,
    }

    let keyboard = Kb::build_keyboard().unwrap();
    let rows = keyboard.inline_keyboard;

    assert_eq!(rows.len(), 2);

    assert_eq!(rows[0][0].text, "A");
    assert!(matches!(rows[0][0].kind, InlineKeyboardButtonKind::CallbackData(_)));

    assert_eq!(rows[1][0].text, "B");
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_same_row() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "A", row = 1)]
        A,
        #[button(text = "B", row = 1)]
        B,
    }

    let keyboard = Kb::build_keyboard().unwrap();
    let rows = keyboard.inline_keyboard;

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), 2);

    assert_eq!(rows[0][0].text, "A");
    assert_eq!(rows[0][1].text, "B");
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_unnamed_fields() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "Value", row = 1)]
        Value(i32),
    }

    let keyboard = Kb::build_keyboard(42).unwrap();
    let btn = &keyboard.inline_keyboard[0][0];

    assert_eq!(btn.text, "Value");

    if let teloxide::types::InlineKeyboardButtonKind::CallbackData(data) = &btn.kind {
        assert!(data.contains("42"));
    } else {
        panic!("Expected callback data");
    }
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_named_fields() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "User", row = 1)]
        User { id: i32, name: String },
    }

    let keyboard = Kb::build_keyboard((5, "john".to_string())).unwrap();
    let btn = &keyboard.inline_keyboard[0][0];

    assert_eq!(btn.text, "User");

    if let teloxide::types::InlineKeyboardButtonKind::CallbackData(data) = &btn.kind {
        assert!(data.contains("5"));
        assert!(data.contains("john"));
    } else {
        panic!("Expected callback data");
    }
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_mixed_variants() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "Static", row = 1)]
        Static,
        #[button(text = "Number", row = 2)]
        Number(i32),
    }

    let keyboard = Kb::build_keyboard(10).unwrap();
    let rows = keyboard.inline_keyboard;

    assert_eq!(rows.len(), 2);

    assert_eq!(rows[0][0].text, "Static");
    assert_eq!(rows[1][0].text, "Number");
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_url_button() {
    use teloxide::types::InlineKeyboardButtonKind;

    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "Go", url = "https://example.com", row = 1)]
        Go,
    }

    let keyboard = Kb::build_keyboard().unwrap();
    let btn = &keyboard.inline_keyboard[0][0];

    assert_eq!(btn.text, "Go");

    match &btn.kind {
        InlineKeyboardButtonKind::Url(url) => {
            assert_eq!(url.as_str(), "https://example.com/");
        }
        _ => panic!("Expected URL button"),
    }
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_switch_inline_query() {
    use teloxide::types::InlineKeyboardButtonKind;

    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "Search", switch_inline_query = "query", row = 1)]
        Search,
    }

    let keyboard = Kb::build_keyboard().unwrap();
    let btn = &keyboard.inline_keyboard[0][0];

    match &btn.kind {
        InlineKeyboardButtonKind::SwitchInlineQuery(q) => {
            assert_eq!(q, "query");
        }
        _ => panic!("Expected switch inline query"),
    }
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_row_ordering() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "Row1", row = 1)]
        A,
        #[button(text = "Row2", row = 2)]
        B,
        #[button(text = "Row2-second", row = 2)]
        C,
    }

    let keyboard = Kb::build_keyboard().unwrap();
    let rows = keyboard.inline_keyboard;

    assert_eq!(rows.len(), 2);

    assert_eq!(rows[0][0].text, "Row1");

    assert_eq!(rows[1].len(), 2);
    assert_eq!(rows[1][0].text, "Row2");
    assert_eq!(rows[1][1].text, "Row2-second");
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_custom_separator() {
    #[derive(InlineButtons, Debug)]
    #[button(fields_separator = "|")]
    enum Kb {
        #[button(text = "Data", row = 1)]
        Data(String),
    }

    let keyboard = Kb::build_keyboard("abc".to_string()).unwrap();
    let btn = &keyboard.inline_keyboard[0][0];

    if let teloxide::types::InlineKeyboardButtonKind::CallbackData(data) = &btn.kind {
        assert!(data.contains('|'));
    } else {
        panic!("Expected callback data");
    }
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_multiple_params_order() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "A", row = 1)]
        A(i32),
        #[button(text = "B", row = 2)]
        B(String),
    }

    let keyboard = Kb::build_keyboard(7, "hello".to_string()).unwrap();

    let rows = keyboard.inline_keyboard;

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0][0].text, "A");
    assert_eq!(rows[1][0].text, "B");
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_param_ordering_cross_variants() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "A", row = 1)]
        A(i32, String),
        #[button(text = "B", row = 2)]
        B(bool),
    }

    let keyboard = Kb::build_keyboard((10, "hello".to_string()), true).unwrap();

    let rows = keyboard.inline_keyboard;

    let a_btn = &rows[0][0];
    let b_btn = &rows[1][0];

    if let teloxide::types::InlineKeyboardButtonKind::CallbackData(data) = &a_btn.kind {
        assert!(data.contains("10"));
        assert!(data.contains("hello"));
    } else {
        panic!("Expected callback data");
    }

    if let teloxide::types::InlineKeyboardButtonKind::CallbackData(data) = &b_btn.kind {
        assert!(data.contains("true"));
    } else {
        panic!("Expected callback data");
    }
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_no_params_for_unit_variants() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "A", row = 1)]
        A,
        #[button(text = "B", row = 2)]
        B,
    }

    // Should compile with zero args
    let keyboard = Kb::build_keyboard().unwrap();

    assert_eq!(keyboard.inline_keyboard.len(), 2);
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_mixed_unit_and_args_order() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "A", row = 1)]
        A,
        #[button(text = "B", row = 2)]
        B(i32),
    }

    let keyboard = Kb::build_keyboard(99).unwrap();

    let rows = keyboard.inline_keyboard;

    assert_eq!(rows[0][0].text, "A");
    assert_eq!(rows[1][0].text, "B");
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_interleaved_rows() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "R1-A", row = 1)]
        A,
        #[button(text = "R2-A", row = 2)]
        B,
        #[button(text = "R1-B", row = 1)]
        C,
    }

    let keyboard = Kb::build_keyboard().unwrap();
    let rows = keyboard.inline_keyboard;

    assert_eq!(rows.len(), 2);

    assert_eq!(rows[0][0].text, "R1-A");

    assert_eq!(rows[0].len(), 2);
    assert_eq!(rows[1][0].text, "R2-A");
    assert_eq!(rows[0][1].text, "R1-B");
}

#[test]
#[cfg(feature = "macros")]
fn keyboard_callback_roundtrip() {
    #[derive(InlineButtons, Debug, PartialEq)]
    enum Kb {
        #[button(text = "Test", row = 1)]
        Test(i32, String),
    }

    let keyboard = Kb::build_keyboard((5, "abc".to_string())).unwrap();
    let btn = &keyboard.inline_keyboard[0][0];

    let data = match &btn.kind {
        teloxide::types::InlineKeyboardButtonKind::CallbackData(d) => d.clone(),
        _ => panic!("Expected callback data"),
    };

    let parsed = Kb::parse(&data).unwrap();

    assert_eq!(parsed, Kb::Test(5, "abc".to_string()));
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_named_fields_ordering() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "User", row = 1)]
        User { id: i32, name: String },
    }

    let keyboard = Kb::build_keyboard((1, "bob".to_string())).unwrap();
    let btn = &keyboard.inline_keyboard[0][0];

    if let teloxide::types::InlineKeyboardButtonKind::CallbackData(data) = &btn.kind {
        let parts: Vec<_> = data.split(';').collect();
        assert_eq!(parts[1], "1");
        assert_eq!(parts[2], "bob");
    } else {
        panic!("Expected callback data");
    }
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_many_variants() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "1", row = 1)]
        A,
        #[button(text = "2", row = 2)]
        B,
        #[button(text = "3", row = 3)]
        C,
        #[button(text = "4", row = 4)]
        D,
        #[button(text = "5", row = 5)]
        E,
        #[button(text = "6", row = 6)]
        F,
        #[button(text = "7", row = 7)]
        G,
        #[button(text = "8", row = 8)]
        H,
    }

    let keyboard = Kb::build_keyboard().unwrap();
    assert_eq!(keyboard.inline_keyboard.len(), 8);
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_default_text() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(row = 1)]
        HelloWorld,
    }

    let keyboard = Kb::build_keyboard().unwrap();
    let btn = &keyboard.inline_keyboard[0][0];

    assert_eq!(btn.text, "HelloWorld");
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_separator_propagation() {
    #[derive(InlineButtons, Debug)]
    #[button(fields_separator = "|")]
    enum Kb {
        #[button(text = "Data", row = 1)]
        Data(i32, i32),
    }

    let keyboard = Kb::build_keyboard((1, 2)).unwrap();
    let btn = &keyboard.inline_keyboard[0][0];

    if let teloxide::types::InlineKeyboardButtonKind::CallbackData(data) = &btn.kind {
        assert!(data.contains('|'));
        assert!(!data.contains(';'));
    } else {
        panic!("Expected callback data");
    }
}

#[test]
#[cfg(feature = "macros")]
fn build_keyboard_no_duplicate_rows() {
    #[derive(InlineButtons, Debug)]
    enum Kb {
        #[button(text = "A", row = 1)]
        A,
        #[button(text = "B", row = 1)]
        B,
        #[button(text = "C", row = 2)]
        C,
    }

    let keyboard = Kb::build_keyboard().unwrap();

    assert_eq!(keyboard.inline_keyboard.len(), 2);
}
