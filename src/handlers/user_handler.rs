use crate::services::user_service::UserService;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn get_user(
    State(service): State<UserService>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    match service.get_user_by_id(id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(err) => (StatusCode::NOT_FOUND, err).into_response(),
    }
}
