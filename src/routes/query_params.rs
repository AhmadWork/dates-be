use axum::{extract::Query, Json};
use serde::{ Serialize, Deserialize };

#[derive(Serialize,Deserialize)]
pub struct QueryParams {
message: String,
id: i16,
}
pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    let response = QueryParams { message: query.message, id: query.id + 10 };
Json(response)
}
