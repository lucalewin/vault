use axum::{extract::FromRequestParts, http::{request::Parts, StatusCode}, RequestPartsExt};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject
    sub: String,
    /// Expiration time
    exp: usize,
}

pub fn generate_token(sub: String) -> String {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(10))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims { sub, exp: exp as usize };
    let header = jsonwebtoken::Header::default();
    jsonwebtoken::encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(b"secret"))
        .expect("valid token")
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(b"secret"),
        &jsonwebtoken::Validation::default(),
    )?;
    Ok(token.claims)    
}

pub struct SessionUser(pub i32);

impl<S> FromRequestParts<S> for SessionUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // Extract the Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Validate the token
        let claims = verify_token(bearer.token()).map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(SessionUser(
            claims
                .sub
                .parse()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        ))
    }
}

