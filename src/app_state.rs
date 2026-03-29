#[cfg(feature = "ssr")]
mod ssr {
    use axum::extract::FromRef;
    use leptos::config::LeptosOptions;
    use moka::sync::Cache;

    #[derive(Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub sessions: Cache<String, String>,
    }

    impl FromRef<AppState> for LeptosOptions {
        fn from_ref(state: &AppState) -> Self {
            state.leptos_options.clone()
        }
    }

    impl FromRef<AppState> for Cache<String, String> {
        fn from_ref(state: &AppState) -> Self {
            state.sessions.clone()
        }
    }
}

#[cfg(feature = "ssr")]
pub use ssr::*;
