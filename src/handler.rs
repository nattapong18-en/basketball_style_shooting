use crate::models::{AppState, CreatePlayer, PlayerProfile, UpdatePlayer};
use axum::http::HeaderMap;
use axum::{
    Json,
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use subtle::ConstantTimeEq;

pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let valid_token = std::env::var("API_TOKEN").unwrap_or_default();

    if let Some(token) = headers.get("authorization") {
        println!("[Debug] Token from client: {:?}", token);
        println!("[Debug] Token from env: {:?}", valid_token);

        if let Ok(token_str) = token.to_str() {
            let is_valid = token_str.as_bytes().ct_eq(valid_token.as_bytes());
            if is_valid.into() {
                return next.run(request).await;
            }
        }
    }
    (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}

pub async fn analyze_player(
    State(state): State<AppState>,
    Json(payload): Json<CreatePlayer>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO players (name, shooting_style) VALUES (?, ?)",
        payload.name,
        payload.shooting_style
    )
    .execute(&state.pool)
    .await;
    match result {
        Ok(db_result) => {
            let new_id = db_result.last_insert_rowid();
            println!("Inserted player with ID {}", new_id);
            let new_player = PlayerProfile {
                id: new_id,
                name: payload.name,
                shooting_style: payload.shooting_style,
            };
            let alert_message = format!(
                "New player analyzed: {} with shooting style {}",
                new_player.name, new_player.shooting_style
            );
            notify_line(&state.http_client, &alert_message).await;
            let feedback = new_player.analyze();
            (StatusCode::CREATED, Json(feedback)).into_response()
        }
        Err(e) => {
            eprintln!("Error inserting player: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert player").into_response()
        }
    }
}
pub async fn get_all_players(State(state): State<AppState>) -> impl IntoResponse {
    let result = sqlx::query_as!(PlayerProfile, "SELECT * FROM players")
        .fetch_all(&state.pool)
        .await;

    match result {
        Ok(player) => (StatusCode::OK, Json(player)).into_response(),
        Err(e) => {
            eprintln!("Error fetching players: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetching players",
            )
                .into_response()
        }
    }
}
pub async fn get_player_history(
    State(state): State<AppState>,
    Path(player_id): Path<i64>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        PlayerProfile,
        "SELECT id, name, shooting_style FROM players WHERE id = ?",
        player_id
    )
    .fetch_optional(&state.pool)
    .await;
    match result {
        Ok(Some(profile)) => (StatusCode::OK, Json(profile)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}
pub async fn get_player_by_name(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        PlayerProfile,
        "SELECT id, name, shooting_style FROM players WHERE LOWER(name) = LOWER(?)",
        name
    )
    .fetch_optional(&state.pool)
    .await;
    match result {
        Ok(Some(player)) => (StatusCode::OK, Json(player)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn update_player(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePlayer>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "UPDATE players SET shooting_style = ? WHERE id = ?",
        payload.shooting_style,
        id
    )
    .execute(&state.pool)
    .await;
    match result {
        Ok(db_result) => {
            if db_result.rows_affected() > 0 {
                (StatusCode::OK, "Player updated successfully").into_response()
            } else {
                (StatusCode::NOT_FOUND, "Player not found").into_response()
            }
        }
        Err(e) => {
            eprintln!("Error updating player: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update player").into_response()
        }
    }
}

pub async fn delete_player(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM players WHERE id = ?", id)
        .execute(&state.pool)
        .await;
    match result {
        Ok(db_result) => {
            if db_result.rows_affected() > 0 {
                (StatusCode::OK, "Player deleted successfully").into_response()
            } else {
                (StatusCode::NOT_FOUND, "Player not found").into_response()
            }
        }
        Err(e) => {
            eprintln!("Error deleting player: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete player").into_response()
        }
    }
}

pub async fn notify_line(client: &reqwest::Client, message: &str) {
    println!("Starting LINE notification...");
    let token = std::env::var("LINE_CHANNEL_TOKEN")
        .unwrap_or_default()
        .trim()
        .to_string();
    let user_id = std::env::var("LINE_USER_ID")
        .unwrap_or_default()
        .trim()
        .to_string();

    if token.is_empty() || user_id.is_empty() {
        println!("LINE notification not configured.");
        return;
    }

    let payload = serde_json::json!({
        "to": user_id,
        "messages": [
            {
                "type": "text",
                "text": message
            }
        ]
    });

    let res = client
        .post("https://api.line.me/v2/bot/message/push")
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await;

    match res {
        Ok(respone) => {
            let status = respone.status();
            let body = respone
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read response body".to_string());
            println!(
                "LINE notification sent successfully! Status: {}, Response: {}",
                status, body
            );
        }
        Err(e) => {
            eprintln!("Failed to send LINE notification: {}", e);
        }
    }
}
