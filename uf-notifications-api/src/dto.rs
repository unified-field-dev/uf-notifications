//! Client/hydrate DTO when SSR is not enabled (SSR re-exports core).

#[cfg(not(feature = "ssr"))]
use serde::{Deserialize, Serialize};

/// Client/hydrate copy of the notification DTO (matches core layout; SSR uses `uf-notifications-core`).
#[cfg(not(feature = "ssr"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationDto {
    /// Notification's unique id.
    pub notification_id: uuid::Uuid,
    /// Notification title.
    pub title: String,
    /// Notification body text.
    pub message: String,
    /// Human-friendly relative time (e.g. `"5m ago"`).
    pub created_at: String,
    /// Optional link the client should navigate to when the notification is clicked.
    pub url: Option<String>,
    /// Whether the recipient has marked this notification as read.
    pub is_read: bool,
}
