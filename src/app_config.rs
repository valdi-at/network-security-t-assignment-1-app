#[cfg(feature = "ssr")]
mod ssr {
    use std::sync::LazyLock;

    use config::{Config, ConfigError, Environment};
    use serde::Deserialize;

    static CONFIG: LazyLock<AppConfig> =
        LazyLock::new(|| AppConfig::new().expect("Failed to load config"));

    #[derive(Debug, Deserialize)]
    pub struct AppConfig {
        pub oidc_name: String,
        pub oidc_client_id: String,
        pub oidc_client_secret: String,
        pub oidc_scopes: String,
        pub oidc_auth_uri: String,
        pub oidc_token_uri: String,
        pub oidc_api_uri: String,
        pub oidc_redirect_uri: String,
    }

    impl AppConfig {
        pub fn new() -> Result<AppConfig, ConfigError> {
            let cfg = Config::builder()
                .add_source(config::File::with_name("config"))
                .add_source(Environment::default())
                .build()?;

            Ok(cfg.try_deserialize()?)
        }
    }

    pub fn config() -> &'static AppConfig {
        &*CONFIG
    }
}

#[cfg(feature = "ssr")]
pub use ssr::*;
