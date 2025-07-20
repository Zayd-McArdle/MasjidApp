use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::hash::Hash;
use std::sync::LazyLock;

pub struct AuthenticatedUser {
    pub username: String,
    pub role: String,
}

pub static KEYS: LazyLock<ApiKeys> =
    LazyLock::new(|| ApiKeys::new(std::env::var("JWT_SECRET").unwrap().as_bytes()));

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Claims {
    // Issuer
    pub iss: String,
    // Subscriber
    pub sub: String,
    // Audience
    pub aud: String,
    // Issued At
    pub iat: usize,
    // Expiration Time
    pub exp: usize,
}

pub enum ClaimsError {
    InvalidToken,
    FailedToCreateToken,
    ExpiredToken,
}
#[derive(Clone, Serialize)]
pub enum AuthorisationError {
    InvalidToken,
    ExpiredToken,
    UnknownError,
}
impl Claims {
    fn is_valid_aud(&self) -> bool {
        !self.aud.is_empty()
    }
    fn is_valid(&self) -> bool {
        !self.sub.is_empty() && self.is_valid_aud() && self.iat < self.exp
    }
    pub fn generate(sub: &str, aud: &str) -> Self {
        //Gets today's date according to UTC timezone
        let issued_at = chrono::Utc::now().timestamp() as usize;

        //The JWT claim expires after 3 days of being issued
        let expiration_date = issued_at + (3 * 24 * 60 * 60);

        Self {
            iss: "MasjidApp".to_owned(),
            sub: sub.to_owned(),
            aud: aud.to_owned(),
            iat: issued_at,
            exp: expiration_date,
        }
    }
    pub fn regenerate(original_claims: Claims) -> Result<Self, ClaimsError> {
        if original_claims.is_valid() {
            return Ok(Self::generate(&original_claims.sub, &original_claims.aud));
        }
        Err(ClaimsError::InvalidToken)
    }
}

pub struct ApiKeys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl ApiKeys {
    pub fn new(secret: &[u8]) -> Self {
        ApiKeys {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub fn generate_token(claims: &Claims) -> Result<String, ClaimsError> {
    let encoded_token_result = jsonwebtoken::encode(&Header::default(), claims, &KEYS.encoding);

    match encoded_token_result {
        Ok(token) => Ok(token),
        Err(err) => {
            tracing::error!("Failed to encode JWT token: {}", err);
            Err(ClaimsError::FailedToCreateToken)
        }
    }
}
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthorisationError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Self::Rejection::InvalidToken)?;

        // Configure validation
        let mut validation = Validation::default();
        // HashSet of valid issuers
        validation.iss = Some(["MasjidApp".into()].into_iter().collect());
        // HashSet of valid audiences
        validation.aud = Some(["Admin".into(), "Imam".into()].into_iter().collect());
        // Only allow HS256
        validation.algorithms = vec![Algorithm::HS256];
        // Check expiration
        validation.validate_exp = true;

        // Decode and validate token
        let token_data_result =
            decode::<Claims>(bearer.token().trim(), &KEYS.decoding, &validation);
        match token_data_result {
            Ok(token_data) => {
                if !token_data.claims.is_valid() {
                    return Err(Self::Rejection::InvalidToken);
                }
                Ok(token_data.claims)
            }
            Err(error) => match error.kind() {
                ErrorKind::InvalidAudience
                | ErrorKind::InvalidToken
                | ErrorKind::InvalidIssuer
                | ErrorKind::InvalidSignature => Err(Self::Rejection::InvalidToken),
                ErrorKind::ExpiredSignature => Err(Self::Rejection::ExpiredToken),
                _ => {
                    tracing::error!("unexpected error has occurred: {}", error);
                    Err(Self::Rejection::UnknownError)
                }
            },
        }
    }
}
impl IntoResponse for AuthorisationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthorisationError::InvalidToken | AuthorisationError::ExpiredToken => {
                (StatusCode::UNAUTHORIZED, "invalid token")
            }
            AuthorisationError::UnknownError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "unknown error")
            }
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
