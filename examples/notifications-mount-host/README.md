# notifications-mount-host

Axum oneshot under **`/notifications`** (session-gated). JSON names match the
`notifications` `uf_app!` id/path, the shell slot (`NotificationBell`), the
backend send API, and the Photon WS path.

Production Leptos hosts mount product `NotificationsRoutes` and link
`uf-notifications` for inventory bell (or override with
`provide_shell_notification_bell`). This example proves the same path + auth +
discovery contract without the SSR/WASM / Orbital graph.

| | |
|---|---|
| **When to use** | First smoke of notifications product mount wiring (path, auth, contract names) |
| **Command** | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-uf-notifications cargo run -p notifications-mount-host` |
| **Success** | Stdout: `notifications_mount_host: OK — /notifications protect + inventory + bell slot` |
| **Look next** | Mount the three crates in a product host; call `send_notification` from backend code |

**Open first:** [`src/main.rs`](src/main.rs)

## Copy into your host

| File | What to take |
|------|----------------|
| This [`Cargo.toml`](Cargo.toml) | Axum oneshot shape (path/auth smoke only) |
| Product mount `Cargo.toml` (below) | The three notification crates with `ssr` / `hydrate` features |
| [`src/main.rs`](src/main.rs) | Protect `/notifications`; keep `app_id` / path / shell-slot names |
| Leptos sketch (below) | Route mount + `ensure_notification_bell_linked` |

### Product mount dependencies

```toml
[dependencies]
uf-notifications-core = { git = "https://github.com/unified-field-dev/uf-notifications", package = "uf-notifications-core", branch = "main" }
uf-notifications-api = { git = "https://github.com/unified-field-dev/uf-notifications", package = "uf-notifications-api", branch = "main", default-features = false }
uf-notifications = { git = "https://github.com/unified-field-dev/unified-field-product", package = "uf-notifications", branch = "main", default-features = false }
uf-product = { git = "https://github.com/unified-field-dev/unified-field-product", package = "uf-product", branch = "main", default-features = false }
uf-integrations = { git = "https://github.com/unified-field-dev/unified-field-product", package = "uf-integrations", branch = "main", default-features = false }

[features]
ssr = [
    "uf-notifications-api/ssr",
    "uf-notifications/ssr",
    "uf-product/ssr",
    "uf-integrations/ssr",
]
hydrate = [
    "uf-notifications-api/hydrate",
    "uf-notifications/hydrate",
    "uf-product/hydrate",
    "uf-integrations/hydrate",
]
```

### Leptos mount sketch

```rust,ignore
use leptos::prelude::*;
use leptos_router::components::Routes;
use uf_notifications::{ensure_notification_bell_linked, NotificationsRoutes};

// Inventory fills HostNotificationBell when offering-notifications is on.
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

Backend create path (Leptos-free):

```rust,ignore
use uf_notifications_core::{send_notification, SendNotification};

send_notification(
    SendNotification {
        user_id: some_user_record_id,
        kind: "leaderboard".into(),
        title: "Leaderboard Update".into(),
        message: "You moved up to #3!".into(),
        url: Some("/high-scores".into()),
        data_json: None,
    },
    &valence,
)
.await?;
```

For shell chrome (layout, fonts, Axum + Leptos boot), copy
[`shell-chrome-host`](https://github.com/unified-field-dev/unified-field-product/tree/main/examples/shell-chrome-host)
from unified-field-product, then add domain core/api + product `uf-notifications`
and either link inventory (`ensure_notification_bell_linked`) or call
`provide_shell_notification_bell`.

## Run

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-uf-notifications
cargo check -p notifications-mount-host
cargo run -p notifications-mount-host
```

**Success:** stdout prints `notifications_mount_host: OK — /notifications protect + inventory + bell slot`.

## Hydrate / browser

Out of gate for this host. Full UI needs a product binary with `cargo-leptos`,
`wasm32`, session chrome, Photon WS at `/ws/notifications`, and Valence for
`send_notification`. Product hosts own the `uf-product` / Orbital graph; this
oneshot does not link them.
