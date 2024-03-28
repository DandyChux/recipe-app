use bcrypt;
use common::schema::feedback::ErrorResponse;

// Hashes a string value, used for hashing passwords
pub fn hash(s: &String) -> Result<String, ErrorResponse>{
    let hashed_password = bcrypt::hash(s, 4)
        .map_err(|e| {
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: e.to_string(),
            };
            
            error_response
        })?;

    Ok(hashed_password)
}

// Verifies a string value against a hashed value
pub fn verify(password: &str, hash: &str) -> bool {
    let parshed_hash = bcrypt::verify(password, hash).unwrap();

    parshed_hash
}