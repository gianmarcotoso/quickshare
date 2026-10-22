use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::{ConnectInfo, FromRequestParts},
    http::{header, request::Parts},
};

pub struct Uploader {
    pub ip: IpAddr,
    pub user_agent: String,
}

impl<S: Send + Sync> FromRequestParts<S> for Uploader {
    type Rejection = <ConnectInfo<SocketAddr> as FromRequestParts<S>>::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ConnectInfo(address) =
            ConnectInfo::<SocketAddr>::from_request_parts(parts, state).await?;

        Ok(Self {
            ip: address.ip().to_canonical(),
            user_agent: parts
                .headers
                .get(header::USER_AGENT)
                .and_then(|value| value.to_str().ok())
                .unwrap_or_default()
                .to_string(),
        })
    }
}
