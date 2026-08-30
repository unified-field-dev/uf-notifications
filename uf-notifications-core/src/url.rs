//! Same-origin notification action URL sanitization.

/// Sanitize a notification action URL the same way Orbital sanitizes referers.
///
/// Only same-origin relative paths are allowed. Protocol-relative URLs,
/// absolute URLs, auth/API endpoints, and `/home` loops are rejected.
/// Also rejects backslash / control-character tricks some browsers treat as
/// protocol-relative, and absolute URLs smuggled after a leading slash
/// (`/https://…`). Invalid values become `None` (caller should fall back to
/// the inbox).
///
/// # Examples
///
/// ```
/// use uf_notifications_core::sanitize_notification_url;
///
/// assert_eq!(
///     sanitize_notification_url(Some("/high-scores".into())).as_deref(),
///     Some("/high-scores"),
/// );
/// assert_eq!(sanitize_notification_url(Some("//evil.example".into())), None);
/// assert_eq!(sanitize_notification_url(Some("/auth/signin".into())), None);
/// ```
pub fn sanitize_notification_url(url: Option<String>) -> Option<String> {
    url.filter(|path| is_safe_notification_path(path))
}

/// True when `path` is a same-origin absolute path suitable for notification navigation.
pub fn is_safe_notification_path(path: &str) -> bool {
    if !path.starts_with('/') || path.starts_with("//") {
        return false;
    }
    // `/\evil.example` is protocol-relative in some browsers.
    if path.contains('\\') {
        return false;
    }
    // Reject ASCII controls / whitespace (tab, CR, LF, NUL, …).
    if path.bytes().any(|b| b <= 0x20 || b == 0x7f) {
        return false;
    }
    // Reject absolute URLs smuggled as paths (`/https://evil.example`).
    if path.contains("://") {
        return false;
    }
    if path.starts_with("/auth/") || path.starts_with("/api/") {
        return false;
    }
    if path == "/home" || path == "/home/" {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{is_safe_notification_path, sanitize_notification_url};

    #[test]
    fn sanitize_notification_url_keeps_safe_paths() {
        assert_eq!(
            sanitize_notification_url(Some("/high-scores".into())).as_deref(),
            Some("/high-scores")
        );
        assert!(is_safe_notification_path("/notifications"));
        assert!(is_safe_notification_path("/counter/admin"));
    }

    #[test]
    fn sanitize_notification_url_rejects_open_redirects() {
        assert_eq!(
            sanitize_notification_url(Some("//evil.example".into())),
            None
        );
        assert_eq!(
            sanitize_notification_url(Some("https://evil.example/x".into())),
            None
        );
        assert_eq!(sanitize_notification_url(Some("/auth/signin".into())), None);
        assert_eq!(sanitize_notification_url(Some("/api/x".into())), None);
        assert_eq!(sanitize_notification_url(Some("/home".into())), None);
        assert_eq!(sanitize_notification_url(None), None);
    }

    #[test]
    fn sanitize_notification_url_rejects_backslash_control_and_url_smuggle_sad() {
        assert_eq!(
            sanitize_notification_url(Some("/\\evil.example".into())),
            None
        );
        assert_eq!(
            sanitize_notification_url(Some("/\tevil.example".into())),
            None
        );
        assert_eq!(
            sanitize_notification_url(Some("/https://evil.example".into())),
            None
        );
        assert_eq!(
            sanitize_notification_url(Some("/counter/admin\n".into())),
            None
        );
        assert!(!is_safe_notification_path("/\u{0000}evil"));
    }
}
