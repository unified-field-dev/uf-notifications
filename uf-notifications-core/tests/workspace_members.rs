//! Gate: core/api + mount-host are members of this domain workspace.
//! UI (`uf-notifications`) lives in unified-field-product.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[test]
fn notifications_workspace_members_happy_path() {
    let root =
        fs::read_to_string(workspace_root().join("Cargo.toml")).expect("workspace Cargo.toml");
    for member in [
        "uf-notifications-core",
        "uf-notifications-api",
        "examples/notifications-mount-host",
    ] {
        assert!(
            root.contains(&format!("\"{member}\"")),
            "workspace must list {member}"
        );
        assert!(
            workspace_root().join(member).join("Cargo.toml").is_file(),
            "missing crate dir {member}"
        );
    }
    assert!(
        !root.contains("\"uf-notifications\""),
        "UI crate must not remain a domain workspace member"
    );
    assert!(
        !root.contains("notifications-ui-e2e"),
        "notifications-ui-e2e must not remain a domain workspace member"
    );
}
