use leptos::prelude::*;

#[component]
pub fn LoginRedirect() -> impl IntoView {
    let action = ServerAction::<Login>::new();

    Effect::new(move |_| {
        action.dispatch(Login {});
    });

    view! {
        <p>"Redirecting..."</p>
    }
}

#[server(Login)]
async fn login() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::app_state::AppState;
        use ::uuid::Uuid;
        let (verifier, challenge) = generate_pkce();
        let state = expect_context::<AppState>();
        let state_str = Uuid::new_v4().to_string();

        state.pkce_store.insert(state_str.clone(), verifier.clone());
        let url = format!(
            "http://localhost:8080/realms/NetSec-T-opensmh/protocol/openid-connect/auth\
            ?response_type=code\
            &client_id=ff0b5ab041fd3c94190547c1fb50c05d5cd4bbcf463dd5f6b1bc11f1c76c90af7d6d0d2e5d8616771fea1884e12d0dfc14bf7c5325b1acb173fbe594f6487908\
            &redirect_uri=http://localhost:3000/callback\
            &scope=openid\
            &state={}\
            &code_challenge={}\
            &code_challenge_method=S256",
            state_str, challenge
        );
        leptos_axum::redirect(&url);
    }

    Ok(())
}

#[cfg(feature = "ssr")]
mod ssr {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    use rand::{rngs::OsRng, RngCore};
    use sha2::{Digest, Sha256};

    pub fn generate_pkce() -> (String, String) {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);

        let verifier = URL_SAFE_NO_PAD.encode(bytes);
        let hash = Sha256::digest(verifier.as_bytes());
        let challenge = URL_SAFE_NO_PAD.encode(hash);

        (verifier, challenge)
    }
}
#[cfg(feature = "ssr")]
pub use ssr::*;
