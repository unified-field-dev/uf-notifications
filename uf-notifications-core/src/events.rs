//! Photon event types for the notification system.

/// Published when a new notification is created for a user.
#[photon::topic(name = "user.notifications", keyed_by = "user_id")]
pub struct NotificationPushed {
    /// The `user_id` this notification belongs to (used as topic key).
    pub user_id: String,
    /// The notification's UUID (as a string).
    pub notification_id: String,
}
