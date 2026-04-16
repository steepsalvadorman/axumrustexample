use std::sync::Arc;
use sqlx::PgPool;
use crate::services::auth_service::AuthService;
use crate::repositories::user_repo::{PostgresUserRepository, UserRepoTrait};

/// Estado compartido de la aplicación.
///
/// Se inyecta en cada handler de Axum mediante `State<AppState>`.
/// Utiliza `Arc` para compartir el servicio de forma segura entre
/// múltiples hilos sin necesidad de copiar los datos.
#[derive(Clone)]
pub struct AppState {
    /// Servicio de autenticación envuelto en `Arc` para compartirse
    /// entre hilos. Usa `Box<dyn UserRepoTrait>` internamente para
    /// permitir inyección de dependencias y facilitar pruebas con mocks.
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    /// Crea el estado de la aplicación a partir del pool de base de datos.
    pub fn new(pool: PgPool) -> Self {
        let repo: Box<dyn UserRepoTrait + Send + Sync> =
            Box::new(PostgresUserRepository { pool });

        let auth_service = Arc::new(AuthService::new(repo));

        Self { auth_service }
    }
}