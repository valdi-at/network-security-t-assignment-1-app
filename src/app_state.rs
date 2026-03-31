#[cfg(feature = "ssr")]
mod ssr {
    use axum::extract::FromRef;
    use base64::Engine;
    use leptos::config::LeptosOptions;
    use moka::sync::Cache;

    #[derive(Clone, Debug)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub sessions: Cache<String, TokenStore>,
        pub pkce_store: Cache<String, String>,
    }

    impl FromRef<AppState> for LeptosOptions {
        fn from_ref(state: &AppState) -> Self {
            state.leptos_options.clone()
        }
    }

    impl FromRef<AppState> for Cache<String, String> {
        fn from_ref(state: &AppState) -> Self {
            state.pkce_store.clone()
        }
    }

    impl FromRef<AppState> for Cache<String, TokenStore> {
        fn from_ref(state: &AppState) -> Self {
            state.sessions.clone()
        }
    }

    use serde::{de::DeserializeOwned, Deserialize, Serialize};
    use std::{
        collections::HashMap,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct TokenStore {
        pub access_token: String,
        pub token_type: String,

        #[serde(default)]
        pub expires_in: Option<u64>,

        #[serde(default)]
        pub refresh_token: Option<String>,

        #[serde(default)]
        pub refresh_expires_in: Option<u64>,

        #[serde(default)]
        pub id_token: Option<String>,

        #[serde(default)]
        pub scope: Option<String>,

        // Catch-all
        #[serde(flatten)]
        pub extra: HashMap<String, serde_json::Value>,
    }

    impl TokenStore {
        pub fn is_expired(&self) -> bool {
            let claims = match decode_jwt_payload::<JwtClaims>(&self.id_token.as_deref().unwrap()) {
                Some(c) => c,
                None => return true,
            };

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            now >= claims.exp
        }
        pub fn get_username(&self) -> Option<String> {
            let claims = match decode_jwt_payload::<JwtClaims>(&self.id_token.as_deref().unwrap()) {
                Some(c) => c,
                None => return None,
            };

            let extra_fields: HashMap<String, serde_json::Value> = claims.extra;

            extra_fields
                .get("preferred_username")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct JwtClaims {
        pub iss: String,
        pub sub: String,
        pub aud: serde_json::Value,
        pub exp: u64,
        pub iat: u64,

        #[serde(default)]
        pub scope: Option<String>,

        #[serde(flatten)]
        pub extra: HashMap<String, serde_json::Value>,
    }

    pub fn decode_jwt_payload<T: DeserializeOwned>(token: &str) -> Option<T> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return None;
        }

        let payload = parts[1];

        let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(payload)
            .ok()?;

        serde_json::from_slice(&decoded).ok()
    }
}

#[cfg(feature = "ssr")]
pub use ssr::*;
