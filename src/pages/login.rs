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
        use crate::{app_config::config, app_state::AppState};
        use ::uuid::Uuid;
        let (verifier, challenge) = generate_pkce();
        let state = expect_context::<AppState>();
        let state_str = Uuid::new_v4().to_string();

        state.pkce_store.insert(state_str.clone(), verifier.clone());
        let url = format!(
            "{}\
            ?response_type=code\
            &client_id={}\
            &redirect_uri={}\
            &scope={}\
            &state={}\
            &code_challenge={}\
            &code_challenge_method=S256",
            config().oidc_auth_uri,
            config().oidc_client_id,
            config().oidc_redirect_uri,
            config().oidc_scopes,
            state_str,
            challenge
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
