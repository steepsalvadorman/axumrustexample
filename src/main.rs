use axum::{
    routing::get,
    Router
};




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let enrutador = Router::new()
    .route("/", get(|| async {"Hola, mundo"}));


    let servicio = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Servidor corriendo en http://localhost:3000");

    axum::serve(servicio, enrutador).await?;

    Ok(())
}
