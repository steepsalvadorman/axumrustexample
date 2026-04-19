use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use oauth2::{
    reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope,
    TokenResponse,
};
use serde::Deserialize;
use tower_sessions::Session;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn keycloak_login(
    State(state): State<AppState>,
    session: Session,
) -> impl IntoResponse {
    // Generar la URL de redirección hacia Keycloak
    let (auth_url, csrf_token) = state.auth.oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    // Guardar el token CSRF en la sesión de forma segura
    session.insert("oauth_state", csrf_token.secret()).await.unwrap();

    Redirect::to(auth_url.as_ref())
}

pub async fn keycloak_callback(
    State(state): State<AppState>,
    session: Session,
    Query(query): Query<AuthRequest>,
) -> impl IntoResponse {
    // 1. Validar el CSRF Token (prevención de ataques)
    let stored_state: Option<String> = session.get("oauth_state").await.unwrap();
    if stored_state.is_none() || stored_state.unwrap() != query.state {
        return Redirect::to("/login-failed?error=invalid_state");
    }

    // 2. Intercambiar el "Code" por el Token con Keycloak
    let token_result = state.auth.oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            // 3. Guardar el token (o el ID del usuario) en nuestra Sesión blindada
            // La cookie es HTTP-Only por lo que Leptos no puede leerla, pero el navegador la enviará.
            session.insert("access_token", token.access_token().secret()).await.unwrap();
            
            // 4. Redirigir de vuelta al Frontend en Leptos
            Redirect::to("http://localhost:8081/dashboard") // Ajustar a tu URL de Leptos
        }
        Err(e) => {
            eprintln!("Error al obtener token de Keycloak: {:?}", e);
            Redirect::to("/login-failed")
        }
    }
}
