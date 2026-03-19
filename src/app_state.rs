use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct AppState {}
pub type AppStateStore = Arc<RwLock<AppState>>;
