use std::collections::BTreeSet;

use teloxide_core::{
    payloads::{GetBusinessAccountGifts, GetBusinessAccountGiftsSetters},
    types::BusinessConnectionId,
};

#[test]
fn business_account_gift_filters_use_current_wire_names() {
    let payload = GetBusinessAccountGifts::new(BusinessConnectionId("business".to_owned()))
        .exclude_limited_upgradable(true)
        .exclude_limited_non_upgradable(true)
        .exclude_from_blockchain(true);
    let value = serde_json::to_value(payload).unwrap();
    let object = value.as_object().unwrap();

    assert_eq!(object["exclude_limited_upgradable"], true);
    assert_eq!(object["exclude_limited_non_upgradable"], true);
    assert_eq!(object["exclude_from_blockchain"], true);
    assert_eq!(
        object.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "business_connection_id",
            "exclude_from_blockchain",
            "exclude_limited_non_upgradable",
            "exclude_limited_upgradable",
        ]),
    );
}
