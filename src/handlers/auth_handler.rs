use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use crate::state::AppState;
use crate::models::dtos::auth::LoginRequest;

/// Handler POST /login
///
/// Recibe las credenciales del usuario, delega la autenticación
/// al servicio y devuelve el código HTTP apropiado:
/// - `200 OK`          → credenciales correctas
/// - `401 Unauthorized` → credenciales incorrectas o usuario no encontrado
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let response = state.auth.service.authenticate(payload).await;

    if response.success {
        (StatusCode::OK, Json(response)).into_response()
    } else {
        (StatusCode::UNAUTHORIZED, Json(response)).into_response()
    }
}