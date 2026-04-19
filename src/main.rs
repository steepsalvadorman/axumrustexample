use axum::http::{HeaderValue, Method};
use axum::{
    Router,
    routing::{get, post},
};
use backend_rust_axum_2026::handlers::{
    auth_handler::login,
    oauth_handler::{keycloak_callback, keycloak_login},
};
use backend_rust_axum_2026::models::entities::user;
use backend_rust_axum_2026::state::AppState;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl, basic::BasicClient};
use sea_orm::{ConnectionTrait, Database, Schema};
use tower_http::cors::CorsLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Cargar variables de entorno desde .env
    dotenvy::dotenv().ok();

    // 2. Conectar al pool de base de datos
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL debe estar definida en el archivo .env");

    let db = Database::connect(&database_url).await?;
    println!("✅ Conexión a la base de datos establecida");

    // MAGIA DE SEAORM: Auto-crear la tabla si no existe
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let mut table_create_stmt = schema.create_table_from_entity(user::Entity);
    let stmt = builder.build(table_create_stmt.if_not_exists());
    db.execute(stmt).await?;
    println!("✅ Esquema sincronizado (auto-creación ejecutada)");

    let redis_url = std::env::var("REDIS_URL").unwrap().to_string();
    let redis_client = redis::Client::open(redis_url)?;

    // Inicializar OAuth2 Client
    let oauth_client_id =
        ClientId::new(std::env::var("OAUTH_CLIENT_ID").expect("Falta OAUTH_CLIENT_ID"));
    let oauth_client_secret =
        ClientSecret::new(std::env::var("OAUTH_CLIENT_SECRET").expect("Falta OAUTH_CLIENT_SECRET"));
    let auth_url =
        AuthUrl::new(std::env::var("OAUTH_AUTH_URL").expect("Falta OAUTH_AUTH_URL")).unwrap();
    let token_url =
        TokenUrl::new(std::env::var("OAUTH_TOKEN_URL").expect("Falta OAUTH_TOKEN_URL")).unwrap();

    let oauth_client = BasicClient::new(
        oauth_client_id,
        Some(oauth_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new(std::env::var("OAUTH_REDIRECT_URL").expect("Falta OAUTH_REDIRECT_URL"))
            .unwrap(),
    );

    // Configurar el Store de Sesiones (Cookies seguras)
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false); // En producción (HTTPS) esto DEBE ser true

    // 3. Construir el estado compartido de la aplicación
    let state = AppState::new(db, redis_client, oauth_client);

    // Configurar CORS para permitir que Leptos (puerto 8081) se comunique con Axum
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(["http://localhost:8081".parse::<HeaderValue>().unwrap()])
        .allow_credentials(true);

    // 4. Definir las rutas
    let enrutador = Router::new()
        .route("/auth/login", post(login))
        .route("/auth/keycloak-login", get(keycloak_login))
        .route("/auth/callback", get(keycloak_callback))
        .layer(session_layer)
        .layer(cors)
        .with_state(state);

    // 5. Iniciar el servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("🚀 Servidor corriendo en http://localhost:3000");

    axum::serve(listener, enrutador).await?;

    Ok(())
}
