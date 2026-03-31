use leptos::prelude::*;

#[server]
pub async fn auth_check() -> Result<bool, ServerFnError> {
    use crate::app_state::AppState;
    use axum::http::request::Parts;
    let parts = expect_context::<Parts>();
    let state = expect_context::<AppState>();

    let cookies = parts.headers.get("cookie").and_then(|v| v.to_str().ok());

    let session_id = cookies.and_then(|c| {
        c.split("; ")
            .find(|kv| kv.starts_with("session_id="))
            .map(|kv| kv.trim_start_matches("session_id=").to_string())
    });

    let token_store = session_id.and_then(|id| state.sessions.get(&id));

    if let Some(token) = token_store {
        return Ok(!token.is_expired());
    } else {
        return Ok(false);
    };
}

#[server]
pub async fn auth_get_username() -> Result<Option<String>, ServerFnError> {
    use crate::app_state::AppState;
    use axum::http::request::Parts;
    let parts = expect_context::<Parts>();
    let state = expect_context::<AppState>();

    let cookies = parts.headers.get("cookie").and_then(|v| v.to_str().ok());

    let session_id = cookies.and_then(|c| {
        c.split("; ")
            .find(|kv| kv.starts_with("session_id="))
            .map(|kv| kv.trim_start_matches("session_id=").to_string())
    });

    let token_store = session_id.and_then(|id| state.sessions.get(&id));

    if let Some(token) = token_store {
        return Ok(token.get_username());
    } else {
        return Ok(None);
    };
}
