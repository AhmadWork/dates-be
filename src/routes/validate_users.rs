
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    user: String,
    pass: String
}

pub async fn validate_users(Json(user):Json<RequestUser>){
dbg!(user);
}
