use eyre::Result;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::Local;
#[derive(Serialize, Deserialize)]
pub struct UserClaim {
    pub username: String,
    pub iat: i64,
}

pub fn create_token(secret: &str, username: &str) -> Result<String> {
    let now = Local::now();
    let iat = now.timestamp();
    let user_claim = UserClaim {
        username: username.to_owned(),
        iat
    };
    let headers = Header::new(jsonwebtoken::Algorithm::HS256);
    let token = encode(
        &headers,
        &user_claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}
