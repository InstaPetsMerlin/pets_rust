use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::Error;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::ops::Add;

const AUTH_SECRET: &[u8; 15] = b"some_secret_key";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn authenticate(token: &str) -> Result<bool, Error> {
    let validation = Validation {
        ..Validation::default()
    };
    // validation.algorithms.contains(&Algorithm::RS256);
    let key = &DecodingKey::from_secret(AUTH_SECRET);
    let _token_data = match decode::<Claims>(&token, key, &validation) {
        Ok(_c) => {
            return Ok(true);
        }
        Err(e) => return Err(e),
    };
}

pub fn generate_token(_username: &str, _pet_name: &str) -> String {
    let exp = SystemTime::now()
        .add(Duration::new(3600,0))
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let claim = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: exp as usize,
    };
    match encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(AUTH_SECRET),
    ) {
        Ok(t) => t,

        Err(_) => panic!("could not generate token"),
    }
}
