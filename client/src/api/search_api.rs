use common::schema::feedback::ErrorResponse;
use common::schema::song::Song;
use reqwasm::http;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlDocument};
use serde::Deserialize;

#[derive(Deserialize)]
struct ShazamApiResponse {

}

/// Search for songs using the API
/// 
/// ### Arguments
/// 
/// * `search_input` - A string containing the search query.
/// 
/// ### Returns
/// 
/// Returns a `Result` with a vector of songs if successful, or an error message if the request fails.
pub async fn api_search_songs(search_input: String) -> Result<Vec<Song>, ErrorResponse> {
    #[cfg(debug_assertions)]
    let api_url = "http://localhost:8000";

    #[cfg(not(debug_assertions))]
    let api_url = std::env!("SERVER_URL");
    // let url = format!("{}/api/songs/search?query={}", api_url, search_input);
    let url = format!("https://shazam.p.rapidapi.com/search?term={}", search_input);

    let response = match http::Request::get(&url)
        .header("x-rapidapi-key", "6ed608e828msh52924a2c14e671ep165feajsnae89d37d165c")
        .header("x-rapidapi-host", "shazam.p.rapidapi.com")
        .send()
        .await {
            Ok(res) => res,
            Err(_) => return Err(ErrorResponse { message: "Failed to make request".to_string(), status: "error".to_string() }),
        };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response);
        }

        return Err(ErrorResponse { message: format!("API error: {}", response.status()), status: response.status().to_string() });
    }

    let res_json = response.json::<Vec<Song>>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err(ErrorResponse { message: "Failed to parse response".to_string(), status: "error".to_string() }),
    }
}