use axum::Json;
use serde::Serialize;
#[derive(Clone, Serialize)]
pub struct JsonRes {
    message: String,
    count: i16,
    user: String
}
pub async fn return_json() -> Json<JsonRes>{
    let data = JsonRes{
        message: "helllo mr.Json".to_owned(),
        count: 14,
        user: "dabest".to_owned()
    };

    Json(data)
}
