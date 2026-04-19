use crate::models::dtos::auth::{LoginRequest, LoginResponse};
use crate::repositories::user_repo::UserRepoTrait;

pub struct AuthService {
    repo: Box<dyn UserRepoTrait + Send + Sync>,
}

impl AuthService {
    pub fn new(repo: Box<dyn UserRepoTrait + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async  fn authenticate(&self, payload: LoginRequest) -> LoginResponse {
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
            Ok(None) => LoginResponse {
                success: false,
                message: "Usuario no encontrado".to_string(),
                token: None,
            },
            Err(e) => {
                eprintln!("❌ Error de base de datos en find_by_email: {e}");
                LoginResponse {
                    success: false,
                    message: "Error interno del servidor".to_string(),
                    token: None,
                }
            }
        }
    }
}
