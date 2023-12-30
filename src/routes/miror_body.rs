use serde::{Serialize, Deserialize};
use axum::Json;
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
message: String
}
pub async fn miror_body(Json(body): Json<RequestBody>) ->  Json<RequestBody> {
    dbg!(&body);
    Json(body)
}
