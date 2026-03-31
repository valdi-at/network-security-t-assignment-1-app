use leptos::prelude::*;
use leptos_router::{components::Redirect, hooks::use_query_map};

#[component]
pub fn CallbackRedirect() -> impl IntoView {
    let query = use_query_map();
    let action = ServerAction::<Callback>::new();

    let state_session_id =
        Memo::new(move |_| query.with(|q| q.get("state").map(|s| s.to_string())))
            .get()
            .unwrap_or("https://google.com".to_owned());
    let code = Memo::new(move |_| query.with(|q| q.get("code").map(|s| s.to_string())))
        .get()
        .unwrap_or("https://google.com".to_owned());

    Effect::new(move |_| {
        action.dispatch(Callback {
            state_session_id: state_session_id.clone(),
            code: code.clone(),
        });
    });

    view! {
        {move || {
            match action.value().get() {
                None => view! {
                    <p>"Processing login..."</p>
                }.into_any(),

                Some(Ok(_)) => view! {
                    <Redirect path="/profile"/>
                }.into_any(),

                Some(Err(e)) => view! {
                    <p>"Login failed: " {e.to_string()}</p>
                }.into_any(),
            }
        }}
    }
}

#[server(Callback)]
async fn callback(state_session_id: String, code: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::http::HeaderValue;
        use leptos_axum::ResponseOptions;
        use reqwest::header::SET_COOKIE;
        use uuid::Uuid;

        use crate::app_config::config;
        use crate::app_state::{decode_jwt_payload, AppState, JwtClaims, TokenStore};

        let state = expect_context::<AppState>();
        println!("appstate: {:?}", &state);
        println!("removing: {:?}", &state_session_id);
        let store = state.pkce_store;
        let verifier = store.remove(&state_session_id);
        println!("verifier: {:?}", &verifier);

        let verifier = verifier.unwrap();

        let client = reqwest::Client::new();

        let params = [
            ("grant_type", "authorization_code"),
            ("code", code.as_str()),
            ("redirect_uri", config().oidc_redirect_uri.as_str()),
            ("client_id", config().oidc_client_id.as_str()),
            ("client_secret", config().oidc_client_secret.as_str()),
            ("code_verifier", &verifier),
        ];

        let res = client
            .post(config().oidc_token_uri.as_str())
            .form(&params)
            .send()
            .await
            .unwrap();

        let status = !res.status().is_success();
        let body = res.text().await.unwrap();
        let token: TokenStore = serde_json::from_str(body.as_str()).unwrap();

        println!("Token: {:?}", &token);

        if status {
            return Err(ServerFnError::new(format!("Token error: {}", body)));
        }

        let resp = use_context::<ResponseOptions>()
            .ok_or_else(|| ServerFnError::new("Missing response context"))?;

        let session_id = Uuid::new_v4().to_string();

        let decoded_jwt: Option<JwtClaims> = decode_jwt_payload(token.id_token.as_deref().unwrap());
        if let Some(jwt) = decoded_jwt {
            println!("JWT: {:?}", jwt);
        } else {
            println!("Failed to parse JWT");
        }

        state.sessions.insert(session_id.clone(), token);

        let cookie = HeaderValue::from_str(
            format!("session_id={}; Path=/; HttpOnly; SameSite=Lax", session_id).as_str(),
        )?;

        resp.insert_header(SET_COOKIE, cookie);
    }

    Ok(())
}
