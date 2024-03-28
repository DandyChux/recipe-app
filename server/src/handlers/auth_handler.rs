use axum::{
    http::{header, Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, 
    Router,
    Extension
};
use axum_extra::extract::cookie::{Cookie, SameSite, CookieJar};
use serde_json::json;
use crate::{
    model::Users, utils::{hash::*, jwt::{decode_token, generate_tokens, AccessClaims, AuthError, JwtTokens, RefreshClaims}}, AppState
};
use tokio::sync::RwLock;
use std::sync::Arc;
use common::schema::user::{ FilteredUser, LoginUserSchema, SignupUserSchema, UserData, UserResponse };
use common::schema::platform::Platform;
use common::schema::feedback::ErrorResponse;

pub async fn register_user_handler(
    state: Extension<Arc<RwLock<AppState>>>,
    Json(payload): Json<SignupUserSchema>
    ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let user_exists: Option<bool> = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(&payload.email.to_owned().to_ascii_lowercase())
        .fetch_one(&state.try_read().unwrap().db)
        .await
        .map_err(|e| {
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: e.to_string(),
            };

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    if let Some(exists) = user_exists {
        if exists {
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: "User with this email already exists".to_string(),
            };

            (StatusCode::CONFLICT, Json(error_response));
        }
    }

    let hashed_password = hash(&payload.password).unwrap();

    let user = sqlx::query_as!(
        Users,
        r#"
        INSERT INTO users (user_id, name, username, email, password, preferred_platform)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
        uuid::Uuid::new_v4(),
        &payload.name.to_owned(),
        &payload.username.to_owned(),
        &payload.email.to_owned().to_ascii_lowercase(),
        &hashed_password,
        payload.preferred_platform.as_ref().map(|p| p.to_string())
    )
    .fetch_one(&state.try_read().unwrap().db)
    .await
    .map_err(|e| {
        let error_response = ErrorResponse {
            status: "fail".to_string(),
            message: format!("Database error: {}", e)
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    // Convert the preferred_platform from Option<String> to Option<Platform>
    let preferred_platform = user.preferred_platform.map(|s| Platform::from(s));

    let user_response = json!(UserResponse {
        status: "success".to_string(),
        message: "User registered successfully".to_string(),
        data: UserData {
            user: FilteredUser {
                user_id: user.user_id,
                name: user.name,
                username: user.username,
                email: user.email,
                preferred_platform: preferred_platform.unwrap_or(Platform::Spotify),
                photo: user.photo.unwrap_or_else(|| "".to_string()),
                createdAt: user.created_at.unwrap(),
                updatedAt: user.updated_at.unwrap()
            }
        }
    });

    Ok(Json(user_response))
}

pub async fn login_user_handler(
    state: Extension<Arc<RwLock<AppState>>>,
    Json(payload): Json<LoginUserSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // Add logic to login a user
    let user = sqlx::query_as!(
        Users,
        "SELECT * FROM users WHERE email = $1",
        &payload.email.to_owned()
    )
    .fetch_optional(&state.try_read().unwrap().db)
    .await
    .map_err(|e| {
        let error_response = ErrorResponse {
            status: "fail".to_string(),
            message: format!("Database error: {}", e)
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = ErrorResponse {
            status: "fail".to_string(),
            message: "Invalid email or password: could not locate user with that email".to_string(),
        };

        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;

    let is_valid = verify(&payload.password, &user.password);

    if !is_valid {
        let error_response = ErrorResponse {
            status: "fail".to_string(),
            message: "Invalid password. Does not match hash".to_string(),
        };

        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let JwtTokens { access_token, refresh_token } = generate_tokens(user);

    let is_production = !cfg!(debug_assertions);
    let access_cookie = Cookie::build(("access_token", access_token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .secure(is_production)
        .http_only(true);

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .secure(is_production)
        .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .secure(is_production)
        .http_only(false);

    let res = json!({
        "status": "success",
        "access_token": access_token,
        "refresh_token": refresh_token
    });

    let mut response = Response::new(res.to_string());
    response.headers_mut()
        .insert(header::SET_COOKIE, access_cookie.to_string().parse().unwrap());
    response.headers_mut()
        .append(header::SET_COOKIE, refresh_cookie.to_string().parse().unwrap());
    response.headers_mut()
        .append(header::SET_COOKIE, logged_in_cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn logout_handler () -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let cookie = Cookie::build(("access_token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({ "status": "success" }).to_string());
    response.headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

pub async fn refresh_token_handler(
    state: Extension<Arc<RwLock<AppState>>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // Extract the refresh token from the cookies
    let refresh_token = match cookies.get("refresh_token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: "No refresh token found".to_string(),
            };

            return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
        }
    };

    // Verify the refresh token and get the user ID
    let user_id = decode_token::<RefreshClaims>(&refresh_token)
        .map_err(|e| {
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: match e {
                    AuthError::InvalidToken => "Invalid refresh token".to_string(),
                    _ => "Unknown error".to_string(),
                },
            };

            (StatusCode::UNAUTHORIZED, Json(error_response))
        })?
        .sub;

    // Generate a new pair of access and refresh tokens
    let user = sqlx::query_as!(
        Users,
        "SELECT * FROM users WHERE user_id = $1",
        uuid::Uuid::parse_str(&user_id).unwrap()
    )
    .fetch_one(&state.try_read().unwrap().db)
    .await
    .map_err(|e| {
        let error_response = ErrorResponse {
            status: "fail".to_string(),
            message: format!("Database error: {}", e)
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let JwtTokens { access_token, refresh_token } = generate_tokens(user);

    // Set the new refresh token in the cookies
    let refresh_token_cookie = Cookie::build(("refresh_token", refresh_token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .secure(!cfg!(debug_assertions))
        .http_only(true);

    // Return the new access and refresh tokens
    let res = json!({
        "status": "success",
        "access_token": access_token,
        "refresh_token": refresh_token
    });

    let mut response = Response::new(res.to_string());
    response.headers_mut()
        .insert(header::SET_COOKIE, refresh_token_cookie.to_string().parse().unwrap());

    Ok(Json(res))
}