use crate::models::dtos::auth::{LoginRequest, LoginResponse};
use crate::repositories::user_repo::UserRepoTrait;

/// Servicio de autenticación de usuarios.
///
/// Contiene la lógica de negocio del login y delega el acceso
/// a datos al repositorio mediante un trait object.
pub struct AuthService {
    repo: Box<dyn UserRepoTrait + Send + Sync>,
}

impl AuthService {
    pub fn new(repo: Box<dyn UserRepoTrait + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn authenticate(&self, payload: LoginRequest) -> LoginResponse {
        let result = self.repo.find_by_email(&payload.email).await;

        match result {
            Ok(Some(user)) => {
                if user.password == payload.password {
                    LoginResponse {
                        success: true,
                        message: "Login exitoso".to_string(),
                        token: Some("Un-jwt-ficticio".to_string()),
                    }
                } else {
                    LoginResponse {
                        success: false,
                        message: "Contraseña incorrecta".to_string(),
                        token: None,
                    }
                }
            }
            _ => LoginResponse {
                success: false,
                message: "Usuario no encontrado".to_string(),
                token: None,
            },
        }
    }
}
