use std::sync::Arc;

use axum::{
    body::Body, extract::Request, http::{header, StatusCode}, middleware::Next, response::IntoResponse, Extension, Json
};

use axum_extra::extract::cookie::CookieJar;
use common::schema::feedback::ErrorResponse;
use jsonwebtoken::{decode, DecodingKey, Validation};
use tokio::sync::RwLock;

use crate::{model::Users, utils::jwt::TokenClaims, AppState};

/// Axum JWT Authentication Middleware.
pub async fn auth(
    cookie_jar: CookieJar,
    Extension(app_state): Extension<Arc<RwLock<AppState>>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = cookie_jar
        .get("token") // We try to get the token from the cookie
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            // Otherwise, we try to get it from the authorization header
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    // If the token is none, we return UNAUTHORIZED.
    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "You are not logged in, please provide token".to_string(),
        };

        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    // If the token does not start with "Bearer ", we return UNAUTHORIZED.
    if !token.starts_with("Bearer ") {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };

        return Err((StatusCode::UNAUTHORIZED, Json(json_error)));
    };

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(app_state.clone().read().await.env.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        // We return UNAUTHORIZED if the token fails validation for some reason.
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };

        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?
    .claims;

    // We get the user ID from the token.
    // We try to parse the ID, stored in the token as a String, as a Uuid.
    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        // If the id is incorrectly formed, we return an error.
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };

        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    // With a valid user_id we verify that the user still exists in the database.
    let client = app_state.clone().read().await.db.clone();
    let user_response = sqlx::query_as::<_, Users>(
        "SELECT * FROM users WHERE user_id = $1",
    )
    .bind(&user_id)
    .fetch_one(&client)
    .await
    .map_err(|err| {
        let json_error = ErrorResponse {
            status: "fail",
            message: format!("Error querying the database: {}", err),
        };

        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    // We insert the user_response into the request extensions.
    req.extensions_mut().insert(user_response);

    // If everything is successful, we call the next middleware.
    Ok(next.run(req).await)
}