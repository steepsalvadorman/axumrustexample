use std::sync::Arc;
use sea_orm::DatabaseConnection;
use redis::Client as RedisClient;
use oauth2::basic::BasicClient;
use crate::services::auth_service::AuthService;
use crate::repositories::user_repo::{PostgresUserRepository, UserRepoTrait};

// Se crea un contexto en la aplicacion para postgres
#[derive(Clone)]
pub struct AuthContext {
    pub service: Arc<AuthService>,
    pub oauth_client: BasicClient,
}

// Implementamos primero la base de datos
impl AuthContext {
    pub fn new(db: DatabaseConnection, oauth_client: BasicClient) -> Self {
        let repo: Box<dyn UserRepoTrait + Send + Sync> = Box::new(PostgresUserRepository { db });
        let service = Arc::new(AuthService::new(repo));
        
        Self { service, oauth_client }
    }
}

// Se crea un contexto para Redis
#[derive(Clone)]
pub struct CacheContext {
    pub client: RedisClient,
}

impl CacheContext {
    pub fn new(client: RedisClient) -> Self {
        Self { client }
    }
}



// Finalmente se usa ambos contextos 
#[derive(Clone)]
pub struct AppState {
    pub auth: AuthContext,
    pub cache: CacheContext,
}

impl AppState {
    // El constructor principal ahora pide ambas conexiones principales
    pub fn new(db: DatabaseConnection, redis_client: RedisClient, oauth_client: BasicClient) -> Self {
        Self { 
            auth: AuthContext::new(db, oauth_client),
            cache: CacheContext::new(redis_client),
        }
    }
}
