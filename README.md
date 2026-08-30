# uf-notifications

[![CI](https://github.com/unified-field-dev/uf-notifications/actions/workflows/ci.yml/badge.svg)](https://github.com/unified-field-dev/uf-notifications/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[GitHub](https://github.com/unified-field-dev/uf-notifications) · `cargo doc -p uf-notifications-core --open`

Domain crates for in-app notifications: Valence persistence + Photon push
(`uf-notifications-core`) and authenticated server functions
(`uf-notifications-api`). The Orbital bell + inbox UI lives in
[unified-field-product](https://github.com/unified-field-dev/unified-field-product)
as the `uf-notifications` package.

```toml
[dependencies]
uf-notifications-core = { git = "https://github.com/unified-field-dev/uf-notifications", package = "uf-notifications-core", branch = "main" }
uf-notifications-api = { git = "https://github.com/unified-field-dev/uf-notifications", package = "uf-notifications-api", branch = "main", default-features = false }
# UI (product workspace):
uf-notifications = { git = "https://github.com/unified-field-dev/unified-field-product", package = "uf-notifications", branch = "main", default-features = false }
```

Domain crates depend on Valence, Photon, Higgs, and lepton-identity. The UI crate
depends on unified-field-product shell composition.

## Workspace

| Crate | Role |
|-------|------|
| [`uf-notifications-core`](uf-notifications-core/) | Leptos-free Valence model, `send_notification`, Photon `NotificationPushed` |
| [`uf-notifications-api`](uf-notifications-api/) | Auth-scoped `#[server]` list / page / mark-read + synced unread count |

UI (`uf-notifications` bell, inbox, `/notifications` routes) is a member of
`unified-field-product`, not this repo.

Crate-root rustdoc owns the Features inventory and get-started guides. Start at
`cargo doc -p uf-notifications-core --open`, then the API crate with
`--features ssr`.

## Mount on a host

1. Depend on core + api from this repo and `uf-notifications` from
   unified-field-product. Enable `ssr` / `hydrate` the same way as other uf-apps.
2. From backend code, call `uf_notifications_core::send_notification` to persist + publish.
3. Link `uf-notifications` with `uf-integrations` `offering-notifications` / `full`
   so `HostNotificationBell` fills via inventory (or call
   `provide_shell_notification_bell` to override).
4. Mount `NotificationsRoutes` under the host `<Routes>` (auth-gated at `/notifications`).

```rust,ignore
use leptos::prelude::*;
use leptos_router::components::Routes;
use uf_notifications::{ensure_notification_bell_linked, NotificationsRoutes};

// Once at App() root (inventory bell; provide_shell_notification_bell only to override):
ensure_notification_bell_linked();

#[component]
fn AppRoutes() -> impl IntoView {
    view! {
        <Routes fallback=|| "not found">
            <NotificationsRoutes />
        </Routes>
    }
}
```

Feature flags hosts commonly enable:

| Need | Features |
|------|----------|
| SSR server functions + Valence | `uf-notifications-api/ssr`, `uf-notifications/ssr` |
| Hydrate / WASM client | `uf-notifications-api/hydrate`, `uf-notifications/hydrate` |
| Dev-only create helper | `dev-tools` on api (and UI if needed); keep off in production |

## Examples

- [`notifications-mount-host`](examples/notifications-mount-host/) — `/notifications` protect, inventory id, shell bell slot names
- Index: [`examples/README.md`](examples/README.md)

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-uf-notifications
cargo run -p notifications-mount-host
cargo test -p uf-notifications-core
cargo test -p uf-notifications-api
```

**Success (host):** stdout prints `notifications_mount_host: OK — /notifications protect + inventory + bell slot`.
Hydrate/browser is out of gate for the oneshot. Bell + inbox UI checks run from
the product workspace (`uf-notifications` / shell-chrome-host).

## Security

See [`SECURITY.md`](SECURITY.md). Notification create allows System jobs and
authenticated product fanout at the Valence layer; production builds still expose
no session create server function. Reads and mark-read are scoped to the
authenticated caller via higgs request context. Action URLs are same-origin
relative paths only (`sanitize_notification_url`), including rejection of
backslash, control-character, and `/https://…` open-redirect bypasses.
List/page `limit` and search query length are capped server-side. Keep
`dev-tools` off in production builds so `create_test_notification` is not compiled
in.

## Verify

See [`docs/VERIFICATION.md`](docs/VERIFICATION.md) for core/api tests, mount-host, and rustdoc gates.

CI runs on every push and PR ([`.github/workflows/ci.yml`](.github/workflows/ci.yml)):
fmt, clippy (`-D warnings`) on core/api + mount-host, core/api tests + mount-host run, and core/api rustdoc with broken intra-doc links denied. No root `deny.toml`, so deny is not in CI.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-uf-notifications
cargo check -p notifications-mount-host
cargo run -p notifications-mount-host
cargo test -p uf-notifications-core
cargo test -p uf-notifications-api
cargo test -p uf-notifications-core --test workspace_members --test product_surface
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p uf-notifications-core --no-deps
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p uf-notifications-api --features ssr --no-deps
```

Workspace `Cargo.toml` allows `broken_intra_doc_links` by default; the deny commands above are the CI bar for core/api. `uf-notifications-api` still uses `#![allow(missing_docs)]` on macro-heavy surfaces — see [`docs/VERIFICATION.md`](docs/VERIFICATION.md).

## FAQ

**Is this a runnable host?** The domain crates are libraries. For a local smoke of
the mount contract, run `cargo run -p notifications-mount-host`. A full UI host
still needs the product `uf-notifications` crate, session chrome, Photon/Higgs, and Valence.

**Where do I create notifications from backend code?** `uf_notifications_core::send_notification` — persists the Valence row and publishes `NotificationPushed` for the recipient.

**How does the bell stay live?** `get_unread_count` is `#[photon_leptos::synced]` on topic `user.notifications` over `/ws/notifications`.

**How do I pin dependencies?** Day-to-day: `git` + `branch = "main"` on
`unified-field-dev` (same shape as other UF workspaces). Use `rev` or `tag` only
when you need a frozen pin.

## License

MIT. See [LICENSE](LICENSE).
