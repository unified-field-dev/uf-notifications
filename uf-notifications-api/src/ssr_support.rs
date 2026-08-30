//! SSR helpers for notification server functions (higgs + higgs-host; no lepton-auth).

use leptos::prelude::*;
use valence::RecordId;

use crate::ops::NotificationOpsError;
use crate::parse_session_user_id;

/// Extract [`higgs::Higgs`] from the current request.
///
/// # Errors
///
/// Returns [`ServerFnError`] with `"request context failed"` when
/// [`higgs::Higgs::from_request`] fails (no Higgs Display text forwarded).
pub async fn higgs_ctx() -> Result<higgs::Higgs, ServerFnError> {
    match higgs::Higgs::from_request().await {
        Ok(ctx) => Ok(ctx),
        Err(_) => Err(ServerFnError::new("request context failed")),
    }
}

/// Map [`higgs::HiggsError`] into a server function error without Higgs Display text.
pub fn map_higgs_err(err: higgs::HiggsError) -> ServerFnError {
    match err {
        higgs::HiggsError::ConfigNotInContext
        | higgs::HiggsError::Internal
        | higgs::HiggsError::SubsystemNotConfigured(_) => {
            ServerFnError::new("request context failed")
        }
    }
}

/// Map [`NotificationOpsError`] to an opaque client-visible [`ServerFnError`].
///
/// Preserves the locked string contract (`"notification store failed"`,
/// `"invalid local midnight"`) without embedding Valence engine Display text.
pub fn map_ops_err(err: NotificationOpsError) -> ServerFnError {
    ServerFnError::new(err.to_string())
}

/// Build a user-scoped Valence instance for the current request.
///
/// # Errors
///
/// Returns [`ServerFnError`] with `"request context failed"` when
/// [`higgs::Higgs::valence`] fails (via [`map_higgs_err`]).
pub fn user_valence(ctx: &higgs::Higgs) -> Result<valence::Valence, ServerFnError> {
    ctx.valence().map_err(map_higgs_err)
}

/// Parse `table:id` session user id into a Valence record id.
fn session_user_record_id(session_user_id: &str) -> Result<RecordId, ServerFnError> {
    let (table, id) = parse_session_user_id(session_user_id)
        .map_err(|_| ServerFnError::new("invalid session user id"))?;
    Ok(RecordId::new(table, id))
}

/// Require an authenticated session user alongside Higgs context.
///
/// # Errors
///
/// Returns [`ServerFnError`] when Higgs context is missing (`"request context failed"`),
/// there is no session user (`"You must be signed in"`), or the session id is not
/// `table:id` (`"invalid session user id"`).
pub async fn require_auth_user() -> Result<(higgs::Higgs, RecordId), ServerFnError> {
    let ctx = higgs_ctx().await?;
    let user_id = require_session_record_id(ctx.session_user_id().map(String::as_str))?;
    Ok((ctx, user_id))
}

/// Map an optional Higgs session user id into a Valence [`RecordId`].
///
/// # Errors
///
/// `"You must be signed in"` when absent; `"invalid session user id"` when malformed.
pub fn require_session_record_id(session_user_id: Option<&str>) -> Result<RecordId, ServerFnError> {
    let user_id_str = session_user_id.ok_or_else(|| ServerFnError::new("You must be signed in"))?;
    session_user_record_id(user_id_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::optional_notification;

    #[test]
    fn map_ops_err_store_and_midnight_opaque() {
        let store = map_ops_err(NotificationOpsError::Store);
        let store_msg = store.to_string();
        assert!(
            store_msg.contains("notification store failed"),
            "got: {store_msg}"
        );
        // Ops maps Valence failures to Store without embedding engine Display.
        let _discarded = valence::Error::Validation("user:demo secret-body".into());
        assert!(!store_msg.contains("user:demo"), "got: {store_msg}");
        assert!(!store_msg.contains("secret-body"), "got: {store_msg}");

        let midnight = map_ops_err(NotificationOpsError::InvalidMidnight);
        assert!(
            midnight.to_string().contains("invalid local midnight"),
            "got: {midnight}"
        );
    }

    #[test]
    fn map_higgs_err_omits_config_display_sad() {
        let mapped = map_higgs_err(higgs::HiggsError::ConfigNotInContext);
        let msg = mapped.to_string();
        assert!(msg.contains("request context failed"), "got: {msg}");
        assert!(!msg.contains("provide_context"), "got: {msg}");
        assert!(!msg.contains("HiggsConfig"), "got: {msg}");
    }

    #[test]
    fn optional_notification_privacy_is_none_sad() {
        let mapped = optional_notification(Err(valence::Error::Privacy(
            "Access denied: no matching allow policy".into(),
        )))
        .expect("privacy maps to Ok");
        assert!(mapped.is_none());
    }

    #[test]
    fn optional_notification_not_found_is_none_sad() {
        let mapped = optional_notification(Err(valence::Error::NotFound("n-1".into())))
            .expect("not found maps to Ok");
        assert!(mapped.is_none());
    }

    #[test]
    fn optional_notification_validation_stays_store_err_sad() {
        let mapped = optional_notification(Err(valence::Error::Validation(
            "user:demo secret-body".into(),
        )));
        let err = mapped.expect_err("validation stays an error");
        let msg = err.to_string();
        assert!(msg.contains("notification store failed"), "got: {msg}");
        assert!(!msg.contains("user:demo"), "got: {msg}");
    }

    #[tokio::test]
    async fn require_auth_user_without_higgs_sad() {
        let Err(err) = require_auth_user().await else {
            panic!("missing Higgs must deny");
        };
        let msg = err.to_string();
        assert!(
            msg.contains("request context failed") || msg.contains("You must be signed in"),
            "got: {msg}"
        );
        assert!(!msg.contains("user:demo"), "got: {msg}");
    }

    #[test]
    fn require_session_record_id_signed_out_sad() {
        let Err(err) = require_session_record_id(None) else {
            panic!("None must deny");
        };
        let msg = err.to_string();
        assert!(
            msg.contains("You must be signed in"),
            "typed signed-out denial, got: {msg}"
        );
    }

    #[test]
    fn require_session_record_id_happy_and_malformed_sad() {
        let id = require_session_record_id(Some("user:alice")).expect("valid");
        assert_eq!(id.to_string(), "user:alice");

        let Err(err) = require_session_record_id(Some("not-a-record")) else {
            panic!("malformed must deny");
        };
        assert!(
            err.to_string().contains("invalid session user id"),
            "got: {err}"
        );
    }
}
