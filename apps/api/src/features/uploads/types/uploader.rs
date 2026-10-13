use std::net::{IpAddr, SocketAddr};

use axum::http::{HeaderMap, header};

use crate::libs::describe_device::describe_device;

pub struct Uploader {
    pub ip: IpAddr,
    pub user_agent: String,
}

impl Uploader {
    pub fn from_request(address: SocketAddr, headers: &HeaderMap) -> Self {
        Self {
            ip: address.ip().to_canonical(),
            user_agent: headers
                .get(header::USER_AGENT)
                .and_then(|value| value.to_str().ok())
                .unwrap_or_default()
                .to_string(),
        }
    }

    pub fn device(&self) -> String {
        describe_device(&self.user_agent)
    }
}
