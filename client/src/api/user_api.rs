use common::schema::feedback::ErrorResponse;
use common::schema::user::{SignupUserSchema, LoginUserSchema, FilteredUser as User, UserData, UserResponse, UserLoginResponse};
use reqwasm::http;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlDocument};

/// Registers a user by sending a POST request to the server.
///
/// ### Arguments
///
/// * `user_data` - A JSON string containing user data.
///
/// ### Returns
///
/// Returns a `Result` with the registered user if successful, or an error message if the request fails.
pub async fn api_register_user(user_data: &str) -> Result<User, String> {
    #[cfg(debug_assertions)]
    let api_url = "http://localhost:8000";

    #[cfg(not(debug_assertions))]
    let api_url = std::env!("SERVER_URL");
    let url = format!("{}/api/auth/register", api_url);

    let response = match http::Request::post(&url)
        .header("Content-Type", "application/json")
        .body(user_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        }

        return Err(format!("API error: {}", response.status()));
    }

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

/// Logs in a user by sending a POST request to the server.
///
/// ### Arguments
///
/// * `credentials` - A JSON string containing user credentials.
///
/// ### Returns
///
/// Returns a `Result` with the login response if successful, or an error message if the request fails.
pub async fn api_login_user(credentials: &str) -> Result<UserLoginResponse, String> {
    #[cfg(debug_assertions)]
    let api_url = "http://localhost:8000";

    #[cfg(not(debug_assertions))]
    let api_url = std::env!("SERVER_URL");
    let url = format!("{}/api/auth/login", api_url);

    let response = match http::Request::post(&url)
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(credentials)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(format!("Failed to make request: {}", e)),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        }
        
        return Err(format!("API error: {}", response.status()));
    }

    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        // Ok(data) => {
        //     let cookie = format!("access_token={}; HttpOnly; path=/; max-age=3600; samesite=lax", data.access_token);
        //     let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
        //     document
        //         .set_cookie(&cookie)
        //         .map_err(|_| format!("Error storing cookie into state"))?;

        //     Ok(data)
        // },
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

/// Refreshes the access token by sending a GET request to the server.
///
/// ### Returns
///
/// Returns a `Result` with the refreshed access token if successful, or an error message if the request fails.
pub async fn api_refresh_access_token() -> Result<UserLoginResponse, String> {
    #[cfg(debug_assertions)]
    let api_url = "http://localhost:8000";

    #[cfg(not(debug_assertions))]
    let api_url = std::env!("SERVER_URL");
    let url = format!("{}/api/auth/refresh", api_url);

    let response = match http::Request::post(&url)
        .header("Content-Type", "application/json")
        // .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        }
        
        return Err(format!("API error: {}", response.status()));
    }

    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

/// Retrieves user information by sending a GET request to the server.
///
/// ### Returns
///
/// Returns a `Result` with the user information if successful, or an error message if the request fails.
pub async fn api_user_info() -> Result<User, String> {
    #[cfg(debug_assertions)]
    let api_url = "http://localhost:8000";

    #[cfg(not(debug_assertions))]
    let api_url = std::env!("SERVER_URL");
    let url = format!("{}/api/user/info", api_url);
    
    let response = match http::Request::get(&url)
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        }
        
        return Err(format!("API error: {}", response.status()));
    }

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

/// Logs out a user by sending a GET request to the server.
///
/// ### Returns
///
/// Returns a `Result` with `()` if successful, or an error message if the request fails.
pub async fn api_logout_user() -> Result<(), String> {
    let response = match http::Request::post("http://localhost:8000/api/auth/logout")
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        }
        
        return Err(format!("API error: {}", response.status()));
    }

    Ok(())
}