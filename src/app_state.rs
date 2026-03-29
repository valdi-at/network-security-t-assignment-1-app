#[cfg(feature = "ssr")]
use axum::extract::FromRef;
use leptos::config::LeptosOptions;
#[cfg(feature = "ssr")]
use moka::sync::Cache;

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub sessions: Cache<String, String>,
}

#[cfg(feature = "ssr")]
impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

#[cfg(feature = "ssr")]
impl FromRef<AppState> for Cache<String, String> {
    fn from_ref(state: &AppState) -> Self {
        state.sessions.clone()
    }
}
