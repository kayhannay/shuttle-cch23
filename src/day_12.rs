use axum::extract::{Path, State};
use axum::http::StatusCode;
use tracing::info;
use crate::AppState;

pub async fn day12_save(State(state): State<AppState>, Path(text): Path<String>) -> Result<(),StatusCode> {
    let mut texts = state.day12.lock().map_err(|_| StatusCode::BAD_REQUEST)?;
    let now = chrono::offset::Utc::now();
    info!("Got text: {} and store it with time {}", text, now);
    texts.insert(text.clone(), now);
    Ok(())
}

pub async fn day12_load(State(state): State<AppState>, Path(text): Path<String>) -> Result<String, StatusCode> {
    info!("Load text: {}", text);
    match state.day12.lock().map_err(|_| StatusCode::BAD_REQUEST)?.get(&text) {
        Some(date) => Ok((chrono::offset::Utc::now()-date).num_seconds().to_string()),
        None => Err(StatusCode::NOT_FOUND)
    }
}