use crate::services::auth_service::{Claims, JwtEncoder};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimsTO {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl From<&Claims> for ClaimsTO {
    fn from(value: &Claims) -> Self {
        Self {
            sub: value.sub.clone(),
            exp: value.exp,
            iat: value.iat,
        }
    }
}
impl JwtEncoder {
    pub fn new(secret_key: String) -> Arc<Self>
    where
        Self: Send + Sync,
    {
        Arc::new(Self {
            encode: Box::new(move |claims| {
                encode(
                    &Header::new(Algorithm::HS256),
                    &ClaimsTO::from(claims),
                    &EncodingKey::from_secret(secret_key.as_bytes()),
                )
                .map_err(|e| format!("Token generation error: {}", e))
            }),
        })
    }
}
