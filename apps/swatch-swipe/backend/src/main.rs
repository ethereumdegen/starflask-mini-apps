mod handlers;
mod models;
mod starflask;

use axum::{
    routing::{delete, get, post},
    Router,
};
use models::SavedPalette;
use starflask::StarflaskClient;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub starflask_client: Option<StarflaskClient>,
    pub saved_palettes: Arc<Mutex<Vec<SavedPalette>>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "swatch_swipe_backend=debug,tower_http=debug".into()),
        )
        .init();

    dotenvy::dotenv().ok();

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3001);

    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "../frontend/dist".into());

    let starflask_client = match (
        std::env::var("STARFLASK_API_URL"),
        std::env::var("STARFLASK_SECRET_KEY"),
        std::env::var("STARFLASK_AGENT_ID"),
    ) {
        (Ok(api_url), Ok(secret_key), Ok(agent_id)) => {
            info!("Starflask integration enabled: {}", api_url);
            Some(StarflaskClient::new(api_url, secret_key, agent_id))
        }
        _ => {
            info!("Starflask integration disabled (missing env vars), using mock mode");
            None
        }
    };

    let state = AppState {
        starflask_client,
        saved_palettes: Arc::new(Mutex::new(Vec::new())),
    };

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse().unwrap(),
            "http://127.0.0.1:5173".parse().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any);

    // API routes
    let api_routes = Router::new()
        .route("/api/generate", post(handlers::generate_palette))
        .route("/api/palettes", get(handlers::list_palettes))
        .route("/api/palettes/save", post(handlers::save_palette))
        .route("/api/palettes/{id}", delete(handlers::delete_palette))
        .layer(cors)
        .with_state(state);

    // Serve frontend static files with SPA fallback to index.html
    let index_file = format!("{}/index.html", static_dir);
    let spa_fallback = ServeDir::new(&static_dir).not_found_service(ServeFile::new(&index_file));

    let app = api_routes.fallback_service(spa_fallback);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to port");

    info!(
        "Swatch Swipe listening on port {} (static files: {})",
        port, static_dir
    );

    axum::serve(listener, app).await.expect("Server error");
}
