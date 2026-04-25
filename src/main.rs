mod handler;
mod models;
use axum::{
    middleware::from_fn,
    routing::{Router, delete, get, post, put},
};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

use std::str::FromStr;
use tokio::net::TcpListener;

use crate::{handler::{
    analyze_player, auth_middleware, delete_player, get_all_players, get_player_by_name,
    get_player_history,update_player,
}, models::AppState};

use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum=debug,tower_http=debug,info".into()),
        )
        .init();

    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;

    let connection_options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await?;
        
    
    let state = AppState {
        pool,
        http_client: reqwest::Client::builder()
            .user_agent("bot-notify")
            .build()?
    };

    sqlx::migrate!()
        .run(&state.pool)
        .await?;
        

    let protected_routes = Router::new()
        .route("/update/{id}", put(update_player))
        .route("/delete/{id}", delete(delete_player))
        .layer(from_fn(auth_middleware));

    let app = Router::new()
        .route("/", get(|| async { "Welcome to Basketball API" }))
        .route("/analyze", post(analyze_player))
        
        .route("/history/{name}", get(get_player_by_name))
        .route("/history/id", get(get_player_history))
        .route("/history", get(get_all_players))
        .layer(TraceLayer::new_for_http())
        .merge(protected_routes)
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("API JSON http://127.0.0.1:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
