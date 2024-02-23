use std::sync::Arc;
use chrono::{
    prelude::*,
    Duration
};

use axum::{
    async_trait, body::Body, extract::{Request, State}, http::{header, request::Parts, HeaderMap, StatusCode}, middleware::Next, response::IntoResponse, Json
};

use axum_extra::{
    extract::cookie::CookieJar,
    TypedHeader
};
use jsonwebtoken::{decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Serialize, Deserialize};
use serde_json::{ Value, json };

use crate::{
    model::Users,
    utils::api_error::ApiError,
    AppState,
};
use common::schema::feedback::ErrorResponse;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
}

impl AuthResponse {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthPayload {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    WrongCredentials,
    MissingCredentials,
    TokenCreation
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        let (status_code, message) = match err {
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token"),
        };

        ApiError {
            status_code,
            message: message.to_owned(),
            error_code: None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claim = TokenClaims {
        iat: now.timestamp() as usize, 
        exp: (now + expire).timestamp() as usize, 
        sub: email
    };
    let secret = std::env::var("JWT_SECRET").unwrap();

    encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR })
}

pub fn decode_jwt(headers: &HeaderMap) -> Result<Option<TokenData<TokenClaims>>, (StatusCode, Json<Value>)> {
    // Retrieve the token from the map of request headers
    let token_header = headers.get("Authorization");

    // Map token if it exists - return error if not
    let token = match token_header {
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Missing header"
                }))
            ));
        }
        Some(header) => header.to_str().unwrap()
    };

    // Return error if the token does not start with "Bearer"
    if !token.starts_with("Bearer ") {
        eprintln!("Token is missing 'Bearer ' prefix");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Token is missing 'Bearer' prefix"
            }))
        ));
    }

    // Attempt to decode token and match the results
    match decode::<TokenClaims>(
        &token[7..],
        &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
        &Validation::new(Algorithm::HS256)
    ) {
        Err(err) => {
            match err.kind() {
                // Handle the specific ExpiredSignature error
                ErrorKind::ExpiredSignature => {
                    eprintln!("JWT expired: {:?}", err);
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "error": "Token expired"
                        }))
                    ))
                }
                _ => {
                    // Handle other decoding errors
                    eprintln!("Error decoding JWT: {:?}", err);
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "error": "Error decoding JWT"
                        }))
                    ))
                }
            }
        }
        Ok(decoded_claims) => Ok(Some(decoded_claims))
    }
}