# Examples

Runnable teaching hosts for the notifications **domain** mount contract.

## Canonical path

### `notifications-mount-host` — `/notifications` protect + inventory + bell slot

**Teaches:** session gate on `/notifications`, `uf_app!` contract names
(`notifications` / `/notifications`), the shell notification-bell slot name
(`NotificationBell`), and the backend send / Photon WS contract names.

**Copy:** host [`Cargo.toml`](notifications-mount-host/Cargo.toml) for the
oneshot shape; product mount deps + Leptos sketch in the host README.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-uf-notifications
cargo run -p notifications-mount-host
```

**Success:** stdout prints `notifications_mount_host: OK — /notifications protect + inventory + bell slot`.

**Hydrate / browser:** not required. This host is an Axum oneshot. Full Leptos
SSR + shell chrome lives in unified-field-product (`uf-notifications`,
`examples/shell-chrome-host`).

**Next step:** Mount product `NotificationsRoutes`, link `uf-notifications` for
inventory bell (or `provide_shell_notification_bell` to override), and call
`send_notification` from backend code.

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`notifications-mount-host`](notifications-mount-host/) | Auth gate + inventory + shell slot names | `cargo run -p notifications-mount-host` | Deny/allow + OK line | Product `uf-notifications` mount |
