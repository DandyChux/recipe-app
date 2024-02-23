use bcrypt;

// Hashes a string value, used for hashing passwords
pub fn hash(s: &String) -> String {
    bcrypt::hash(s, 4).unwrap()
}

// Verifies a string value against a hashed value
pub fn verify(s: &str, h: &str) -> bool {
    bcrypt::verify(s, h).unwrap()
}