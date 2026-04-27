use axum::extract::FromRef;
use serde::{Deserialize, Serialize};
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub http_client: reqwest::Client,
}

impl FromRef<AppState> for sqlx::SqlitePool {
    fn from_ref(state: &AppState) -> sqlx::SqlitePool {
        state.pool.clone()
    }
}

impl FromRef<AppState> for reqwest::Client {
    fn from_ref(state: &AppState) -> Self {
        state.http_client.clone()
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerProfile {
    pub id: i64,
    pub name: String,
    pub shooting_style: String,
}
impl PlayerProfile {
    pub fn analyze(&self) -> Coachfeedback {
        let advice = if self.shooting_style == "One-motion" {
            "Keep it style bro"
        } else {
            "Nice shooting from"
        };
        Coachfeedback {
            id: self.id,
            status: "Analyze".to_string(),
            advice: advice.to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreatePlayer {
    pub name: String,
    pub shooting_style: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Coachfeedback {
    pub id: i64,
    pub status: String,
    pub advice: String,
}
#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct UpdatePlayer {
    pub shooting_style: String,
}
