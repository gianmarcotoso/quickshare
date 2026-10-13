use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use chrono::{DateTime, Utc};
use rand::{RngCore, rngs::OsRng};

use crate::features::uploads::types::batch::Batch;

const TOKEN_BYTES: usize = 10;

pub struct Session {
    token: String,
    started_at: DateTime<Utc>,
    executable_sha256: String,
    batches: Mutex<HashMap<String, Arc<Batch>>>,
}

impl Session {
    pub fn new(executable_sha256: String) -> Self {
        let mut token = [0u8; TOKEN_BYTES];
        OsRng.fill_bytes(&mut token);

        Self {
            token: hex::encode(token),
            started_at: Utc::now(),
            executable_sha256,
            batches: Mutex::new(HashMap::new()),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn started_at(&self) -> DateTime<Utc> {
        self.started_at
    }

    pub fn executable_sha256(&self) -> &str {
        &self.executable_sha256
    }

    pub fn batches(&self) -> MutexGuard<'_, HashMap<String, Arc<Batch>>> {
        self.batches
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}
