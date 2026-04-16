use axum::{routing::post, Router};
use backend_rust_axum_2026::handlers::auth_handler::login;
use backend_rust_axum_2026::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Cargar variables de entorno desde .env
    dotenvy::dotenv().ok();

    // 2. Conectar al pool de base de datos
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar definida en el archivo .env");

    let pool = sqlx::PgPool::connect(&database_url).await?;

    println!("✅ Conexión a la base de datos establecida");

    // 3. Construir el estado compartido de la aplicación
    let state = AppState::new(pool);

    // 4. Definir las rutas
    let enrutador = Router::new()
        .route("/auth/login", post(login))
        .with_state(state);

    // 5. Iniciar el servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("🚀 Servidor corriendo en http://localhost:3000");

    axum::serve(listener, enrutador).await?;

    Ok(())
}
