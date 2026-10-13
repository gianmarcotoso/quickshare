use std::sync::Arc;

use crate::core::{app_config::AppConfig, session::Session};

#[derive(Clone)]
pub struct AppState {
    config: Arc<AppConfig>,
    session: Arc<Session>,
}

impl AppState {
    pub fn new(config: AppConfig, session: Session) -> Self {
        Self {
            config: Arc::new(config),
            session: Arc::new(session),
        }
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn session(&self) -> &Session {
        &self.session
    }
}
