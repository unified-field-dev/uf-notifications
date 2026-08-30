use serde::{Deserialize, Serialize};

/// Maximum page size accepted by notification list/page server functions.
///
/// Caps resource abuse from forged `limit` on authenticated list endpoints
/// while remaining above the UI page sizes (bell / inbox use ≤20).
pub const MAX_NOTIFICATION_PAGE_LIMIT: u32 = 50;

/// Maximum rows a count query will load.
///
/// Count endpoints have no SQL `COUNT(*)` on the generated query builder, so
/// they fetch this many ids and return `len`. A full inbox larger than this
/// reports the cap.
pub const MAX_NOTIFICATION_COUNT_CAP: u32 = 500;

/// Clamp a fetched count into `0..=MAX_NOTIFICATION_COUNT_CAP`.
pub fn cap_notification_count(fetched: usize) -> usize {
    fetched.min(MAX_NOTIFICATION_COUNT_CAP as usize)
}

/// Clamp a client-supplied page `limit` into `1..=MAX_NOTIFICATION_PAGE_LIMIT`.
///
/// Zero becomes `1` so callers always receive a positive page size.
pub fn clamp_notification_page_limit(limit: u32) -> u32 {
    limit.clamp(1, MAX_NOTIFICATION_PAGE_LIMIT)
}

/// Maximum free-text search length accepted by [`crate::get_notifications_page`].
pub const MAX_NOTIFICATION_SEARCH_CHARS: usize = 256;

/// Truncate a search query to [`MAX_NOTIFICATION_SEARCH_CHARS`].
pub fn truncate_notification_search(query: &str) -> String {
    let trimmed = query.trim();
    if trimmed.chars().count() <= MAX_NOTIFICATION_SEARCH_CHARS {
        return trimmed.to_string();
    }
    trimmed
        .chars()
        .take(MAX_NOTIFICATION_SEARCH_CHARS)
        .collect()
}

/// Parse a higgs session user id (`table:id`) into table and id parts.
///
/// Used by SSR auth helpers; kept feature-free so unit tests can cover the
/// happy and sad parse contracts without a full request context.
///
/// # Errors
///
/// Returns [`SessionUserIdError`] when the value is missing `:`, or either side
/// is empty.
pub fn parse_session_user_id(session_user_id: &str) -> Result<(&str, &str), SessionUserIdError> {
    match session_user_id.split_once(':') {
        Some((table, id)) if !table.is_empty() && !id.is_empty() => Ok((table, id)),
        _ => Err(SessionUserIdError {
            value: session_user_id.to_string(),
        }),
    }
}

/// Session user id did not match the `table:id` contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionUserIdError {
    /// The rejected session user id (typically `user:…`; never a secret).
    pub value: String,
}

impl std::fmt::Display for SessionUserIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid session user id: {}", self.value)
    }
}

impl std::error::Error for SessionUserIdError {}

/// Read-status filter for notification queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationReadFilter {
    /// No filtering — return notifications regardless of read status.
    All,
    /// Only notifications that have not been marked read.
    Unread,
    /// Only notifications that have been marked read.
    Read,
}
