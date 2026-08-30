//! Auth-scoped notification read paths (`list` / page / counts).

use leptos::prelude::*;
use orbital_paging::Page;

use crate::{NotificationDto, NotificationReadFilter};

#[cfg(feature = "ssr")]
use crate::ops;
#[cfg(feature = "ssr")]
use crate::ssr_support::{map_ops_err, require_auth_user, user_valence};

/// List all notifications for the current user
///
/// # Errors
///
/// Returns [`ServerFnError`] when the caller is not signed in (`"You must be signed in"`),
/// the session user id is malformed (`"invalid session user id"`), Higgs context is
/// missing (`"request context failed"`), or the Valence query fails (`"notification store failed"`).
/// Valence engine display text is not forwarded. [`ServerFnError`] wraps these as opaque
/// client-visible strings.
///
/// # Examples
///
/// ```rust,ignore
/// use uf_notifications_api::list_notifications;
///
/// let recent = list_notifications().await?;
/// ```
#[server]
pub async fn list_notifications() -> Result<Vec<NotificationDto>, ServerFnError> {
    let (ctx, user_id) = require_auth_user().await?;
    let v = user_valence(&ctx)?;
    ops::list_for_user(&v, user_id).await.map_err(map_ops_err)
}

/// Get unread notifications preview (for bell dropdown)
///
/// # Errors
///
/// Same auth and store semantics as [`list_notifications`].
#[server]
pub async fn get_unread_notifications_preview() -> Result<Vec<NotificationDto>, ServerFnError> {
    let (ctx, user_id) = require_auth_user().await?;
    let v = user_valence(&ctx)?;
    ops::unread_preview_for_user(&v, user_id)
        .await
        .map_err(map_ops_err)
}

/// Get a page of unread notifications (for bell dropdown with infinite scroll).
///
/// # Errors
///
/// Same auth and store semantics as [`list_notifications`]. `offset` and `limit` are
/// clamped server-side; invalid values do not produce a separate error.
#[server]
pub async fn get_unread_notifications_page(
    /// Zero-based index of the first notification to return.
    offset: u32,
    /// Maximum number of notifications to return.
    limit: u32,
) -> Result<Page<NotificationDto>, ServerFnError> {
    let (ctx, user_id) = require_auth_user().await?;
    let v = user_valence(&ctx)?;
    ops::unread_page_for_user(&v, user_id, offset, limit)
        .await
        .map_err(map_ops_err)
}

/// Get count of unread notifications.
///
/// Loads at most [`crate::MAX_NOTIFICATION_COUNT_CAP`] rows and returns that
/// length. Larger inboxes report the cap.
///
/// When `ssr` or `hydrate` is enabled, `#[photon_leptos::synced]` subscribes to
/// topic `user.notifications` on `/ws/notifications` and emits
/// [`subscribe_get_unread_count`] so clients refetch after `NotificationPushed`.
///
/// # Errors
///
/// Same auth and store semantics as [`list_notifications`].
///
/// # Examples
///
/// ```rust,ignore
/// use uf_notifications_api::get_unread_count;
///
/// // Signed-in client; host must mount Photon WS at /ws/notifications.
/// let count = get_unread_count().await?;
/// ```
#[cfg_attr(
    any(feature = "ssr", feature = "hydrate"),
    photon_leptos::synced(
        topic = "user.notifications",
        ws = "/ws/notifications",
        strategy = "refetch",
        auth = "user"
    )
)]
#[server]
pub async fn get_unread_count() -> Result<usize, ServerFnError> {
    let (ctx, user_id) = require_auth_user().await?;
    let v = user_valence(&ctx)?;
    ops::unread_count_for_user(&v, user_id)
        .await
        .map_err(map_ops_err)
}

/// Fallback subscription helper for builds with neither `ssr` nor `hydrate`.
///
/// When either feature is enabled, `#[photon_leptos::synced]` on
/// [`get_unread_count`] generates the real [`subscribe_get_unread_count`]
/// helper. In feature-less builds the macro is compiled out entirely, so this
/// stub keeps the API stable: the trigger stays at `0` and the callback is
/// never invoked (matching the macro's no-WebSocket behavior).
///
/// The bell wires this trigger to a `Resource` that calls [`get_unread_count`];
/// each Photon push on `user.notifications` bumps the trigger and refetches.
#[cfg(not(any(feature = "ssr", feature = "hydrate")))]
pub fn subscribe_get_unread_count(_on_event: impl Fn() + Send + Sync + 'static) -> RwSignal<u64> {
    RwSignal::new(0u64)
}

/// Get count of notifications created today
///
/// # Errors
///
/// Same auth and store semantics as [`list_notifications`], plus
/// `"invalid local midnight"` when the host cannot construct today's start
/// timestamp (extremely rare platform edge).
#[server]
pub async fn get_today_count() -> Result<usize, ServerFnError> {
    let (ctx, user_id) = require_auth_user().await?;
    let v = user_valence(&ctx)?;
    ops::today_count_for_user(&v, user_id)
        .await
        .map_err(map_ops_err)
}

/// Get a page of notifications for the current user with server-side search and read-status filtering.
///
/// `read_filter` and `query` are applied server-side; they do not produce
/// separate validation errors. Search text is truncated to
/// [`crate::MAX_NOTIFICATION_SEARCH_CHARS`]; `limit` is clamped via
/// [`crate::clamp_notification_page_limit`].
///
/// # Errors
///
/// Same auth and store semantics as [`list_notifications`].
///
/// # Examples
///
/// ```rust,ignore
/// use uf_notifications_api::{get_notifications_page, NotificationReadFilter};
///
/// let page = get_notifications_page(
///     0,
///     20,
///     Some("leaderboard".into()),
///     NotificationReadFilter::Unread,
/// )
/// .await?;
/// ```
#[server]
pub async fn get_notifications_page(
    /// Zero-based index of the first notification to return.
    offset: u32,
    /// Maximum number of notifications to return.
    limit: u32,
    /// Optional free-text search matched against title and message.
    query: Option<String>,
    /// Read-status filter to apply.
    read_filter: NotificationReadFilter,
) -> Result<Page<NotificationDto>, ServerFnError> {
    let (ctx, user_id) = require_auth_user().await?;
    let v = user_valence(&ctx)?;
    ops::notifications_page_for_user(&v, user_id, offset, limit, query, read_filter)
        .await
        .map_err(map_ops_err)
}

/// Get total notification count for the current user.
///
/// # Errors
///
/// Same auth and store semantics as [`list_notifications`].
#[server]
pub async fn get_notification_count() -> Result<usize, ServerFnError> {
    let (ctx, user_id) = require_auth_user().await?;
    let v = user_valence(&ctx)?;
    ops::notification_count_for_user(&v, user_id)
        .await
        .map_err(map_ops_err)
}
