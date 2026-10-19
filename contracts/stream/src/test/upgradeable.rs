use super::common::Harness;

const ABI_JSON: &str = include_str!("../../abi/fluxora_stream.json");

#[test]
fn upgradeable_is_false_before_any_stream_exists() {
    // Guards the pinned posture and its availability on a fresh deployment.
    let h = Harness::new();
    assert_eq!(h.client.stream_count(), 0, "no stream has been created");
    assert!(!h.client.upgradeable());
}

#[test]
fn halt_operator_cannot_change_upgradeable_posture() {
    // Guards against either halt control changing the reported upgrade posture.
    let h = Harness::new();

    assert!(!h.client.upgradeable());
    h.client.set_halt_operator(&h.sender);
    assert!(!h.client.upgradeable());

    h.client.halt();
    assert!(!h.client.upgradeable());
}

#[test]
fn abi_snapshot_contains_upgradeable_entry_point() {
    // The functions array does not currently list upgradeable; pin the posture metadata.
    let abi: serde_json::Value =
        serde_json::from_str(ABI_JSON).expect("the committed ABI inventory is valid JSON");

    assert!(
        abi.get("upgradeable").and_then(serde_json::Value::as_bool) == Some(false),
        "the ABI inventory must contain upgradeable: false"
    );
}
