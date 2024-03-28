use std::sync::Arc;
use chrono::{
    prelude::*,
    Duration
};

use axum::{
    async_trait, body::Body, extract::{FromRef, FromRequestParts, Request, State}, http::{header, request::Parts, HeaderMap, StatusCode}, middleware::Next, response::IntoResponse, Json, RequestPartsExt
};

use axum_extra::{
    extract::cookie::CookieJar, headers::{authorization::Bearer, Authorization}, TypedHeader
};
use jsonwebtoken::{decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Serialize, Deserialize};
use serde_json::{ Value, json };

use crate::{
    model::Users,
    utils::api_error::ApiError,
    AppState,
    config::Config
};
use common::schema::feedback::ErrorResponse;
use uuid::Uuid;

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

/// [JWT Claims]
/// [RFC7519](https://datatracker.ietf.org/doc/html/rfc7519#section-4)
/// ToDo: implement role based validation: is_role(admin)
/// roles, groups: https://www.rfc-editor.org/rfc/rfc7643.html#section-4.1.2
/// https://www.rfc-editor.org/rfc/rfc9068.html#name-authorization-claims

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    /// The "exp" (expiration time) claim identifies the expiration time on or after which the JWT MUST NOT be accepted for processing.
    pub exp: usize,
    /// The "iat" (issued at) claim identifies the time at which the JWT was issued.
    pub iat: usize,
    /// The "sub" (subject) claim identifies the principal that is the subject of the JWT.
    pub sub: String,
    /// The "jti" (JWT ID) claim provides a unique identifier for the JWT.
    pub jti: String,
    // /// The "role" claim identifies the roles of the user.
    // pub role: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    /// The "exp" (expiration time) claim identifies the expiration time on or after which the JWT MUST NOT be accepted for processing.
    pub exp: usize,
    /// The "iat" (issued at) claim identifies the time at which the JWT was issued.
    pub iat: usize,
    /// The "sub" (subject) claim identifies the principal that is the subject of the JWT.
    pub sub: String,
    /// The "jti" (JWT ID) claim provides a unique identifier for the JWT.
    pub jti: String,
    /// The "reference token" claim references the access token that was used to generate the refresh token.
    pub prf: String,
    /// The "previous expiration" claim identifies the expiration time of the access token that was used to generate the refresh token.
    pub pex: usize,
    // /// The "role" claim identifies the roles of the user.
    // pub role: String
}

pub trait ClaimsMethod {
    // fn validate_role(&self) -> Result<(), AuthError>;
    fn get_sub(&self) -> &str;
    fn get_jti(&self) -> &str;
    fn get_exp(&self) -> usize;
    fn get_iat(&self) -> usize;
    // fn get_role(&self) -> &str;
}

impl ClaimsMethod for AccessClaims {
    // fn validate_role(&self) -> Result<(), AuthError> {
    //     is_role_admin(&self.role)
    // }

    fn get_sub(&self) -> &str {
        &self.sub
    }

    fn get_jti(&self) -> &str {
        &self.jti
    }

    fn get_exp(&self) -> usize {
        self.exp
    }

    fn get_iat(&self) -> usize {
        self.iat
    }

    // fn get_role(&self) -> &str {
    //     &self.role
    // }
}

impl ClaimsMethod for RefreshClaims {
    // fn validate_role(&self) -> Result<(), AuthError> {
    //     is_role_admin(&self.role)
    // }

    fn get_sub(&self) -> &str {
        &self.sub
    }

    fn get_jti(&self) -> &str {
        &self.jti
    }

    fn get_exp(&self) -> usize {
        self.exp
    }

    fn get_iat(&self) -> usize {
        self.iat
    }

    // fn get_role(&self) -> &str {
    //     &self.role
    // }
}

fn is_role_admin(role: &str) -> Result<(), AuthError> {
    if role != "admin" {
        return Err(AuthError::WrongCredentials);
    }
    Ok(())
}

#[async_trait]
impl<S> FromRequestParts<S> for AccessClaims
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        decode_token_from_request_parts::<S, AccessClaims>(parts, state).await
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RefreshClaims
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        decode_token_from_request_parts::<S, RefreshClaims>(parts, state).await
    }
}

async fn decode_token_from_request_parts<S, T>(
    parts: &mut Parts,
    state: &S
) -> Result<T, ApiError>
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
    T: for<'de> Deserialize<'de> + std::fmt::Debug + ClaimsMethod
{
    // extract the token from the authorization header
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| {
            AuthError::MissingCredentials
        })?;

    // decode the token
    let claims = decode_token::<T>(bearer.token())?;

    Ok(claims)
}

pub fn decode_token<T: for<'de> Deserialize<'de>>(
    token: &str
) -> Result<T, AuthError> {
    let config = Config::init();

    let mut validation = Validation::new(Algorithm::HS256);
    // validation.leeway = config.jwt_validation_leeway as u64;

    let token_data = decode::<T>(token, &DecodingKey::from_secret(config.jwt_secret.as_ref()), &validation)
        .map_err(|_| AuthError::WrongCredentials)?;

    Ok(token_data.claims)
}

pub struct JwtTokens {
    pub access_token: String,
    pub refresh_token: String,
}

// pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
//     let now = Utc::now();
//     let expire = Duration::hours(24);

//     let claim = AccessClaims {
//         iat: now.timestamp() as usize, 
//         exp: (now + expire).timestamp() as usize, 
//         sub: email
//     };
//     let secret = std::env::var("JWT_SECRET").unwrap();

//     encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref()))
//         .map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR })
// }

pub fn generate_tokens(user: Users) -> JwtTokens {
    let config = Config::init();

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let sub = user.user_id.to_string();

    let access_token_id = Uuid::new_v4().to_string();
    let refresh_token_id = Uuid::new_v4().to_string();
    let access_token_exp = (now + Duration::seconds(config.jwt_maxage.into())).timestamp() as usize;

    let access_token_claims = AccessClaims {
        iat,
        exp: access_token_exp,
        sub: sub.clone(),
        jti: access_token_id.clone(),
    };

    let refresh_claims = RefreshClaims {
        sub,
        jti: refresh_token_id,
        iat,
        exp: (now + Duration::seconds(config.jwt_maxage.into())).timestamp() as usize,
        prf: access_token_id,
        pex: access_token_exp
    };

    let access_token = encode(
        &Header::default(),
        &access_token_claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref())
    ).unwrap();

    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref())
    ).unwrap();

    JwtTokens {
        access_token,
        refresh_token
    }
}

// pub fn decode_jwt(headers: &HeaderMap) -> Result<Option<TokenData<AccessClaims>>, (StatusCode, Json<Value>)> {
//     // Retrieve the token from the map of request headers
//     let token_header = headers.get("Authorization");

//     // Map token if it exists - return error if not
//     let token = match token_header {
//         None => {
//             return Err((
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({
//                     "error": "Missing header"
//                 }))
//             ));
//         }
//         Some(header) => header.to_str().unwrap()
//     };

//     // Return error if the token does not start with "Bearer"
//     if !token.starts_with("Bearer ") {
//         eprintln!("Token is missing 'Bearer ' prefix");
//         return Err((
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json(json!({
//                 "error": "Token is missing 'Bearer' prefix"
//             }))
//         ));
//     }

//     // Attempt to decode token and match the results
//     match decode::<AccessClaims>(
//         &token[7..],
//         &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
//         &Validation::new(Algorithm::HS256)
//     ) {
//         Err(err) => {
//             match err.kind() {
//                 // Handle the specific ExpiredSignature error
//                 ErrorKind::ExpiredSignature => {
//                     eprintln!("JWT expired: {:?}", err);
//                     Err((
//                         StatusCode::UNAUTHORIZED,
//                         Json(json!({
//                             "error": "Token expired"
//                         }))
//                     ))
//                 }
//                 _ => {
//                     // Handle other decoding errors
//                     eprintln!("Error decoding JWT: {:?}", err);
//                     Err((
//                         StatusCode::UNAUTHORIZED,
//                         Json(json!({
//                             "error": "Error decoding JWT"
//                         }))
//                     ))
//                 }
//             }
//         }
//         Ok(decoded_claims) => Ok(Some(decoded_claims))
//     }
// }