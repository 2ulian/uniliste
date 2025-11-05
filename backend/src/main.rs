 use std::net::SocketAddr;

use axum::{
    extract::{Query, State},
     http::StatusCode,
     response::IntoResponse,
     routing::{get},
     Json, Router,
 };
use serde::{Deserialize, Serialize};
 use tower_http::{cors::CorsLayer, trace::TraceLayer};
 use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

 #[derive(Clone, Default)]
 struct AppState {}

 #[derive(Serialize)]
 struct HealthResponse {
     status: &'static str,
 }

 async fn healthz() -> impl IntoResponse {
     (StatusCode::OK, Json(HealthResponse { status: "ok" }))
 }

 async fn root(State(_state): State<AppState>) -> impl IntoResponse {
     (StatusCode::OK, "Uniliste backend running")
 }

#[derive(Deserialize)]
struct GreetParams {
    name: String,
}

async fn greet_http(Query(params): Query<GreetParams>) -> impl IntoResponse {
    let name = if params.name.trim().is_empty() { "World".to_string() } else { params.name };
    (StatusCode::OK, format!("Hello, {}! You've been greeted from Rust (Axum)!", name))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
     tracing_subscriber::registry()
         .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "uniliste_backend=debug,axum=info,tower_http=info".into()))
         .with(tracing_subscriber::fmt::layer())
         .init();

     let state = AppState::default();

     let app = Router::new()
         .route("/", get(root))
         .route("/healthz", get(healthz))
        .route("/greet", get(greet_http))
         .with_state(state)
         .layer(CorsLayer::permissive())
         .layer(TraceLayer::new_for_http());

     let addr: SocketAddr = "0.0.0.0:8000".parse()?;
    println!("[uniliste-backend] starting...");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("[uniliste-backend] bound on {}", addr);
    tracing::info!("listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
 }