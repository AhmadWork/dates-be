use axum::extract::Path;
pub async fn path_vars(Path(id): Path<i32>) -> String {
    id.to_string()
}
