use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use axum_template::RenderHtml;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::AppState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HtmlContent {
    pub content: String,
}

pub async fn day14_unsafe(State(state): State<AppState>, Json(html_content): Json<HtmlContent>) -> impl IntoResponse {
    info!("Get unsafe called with content {:?}.", html_content);
    let trimmed = HtmlContent { content: html_content.content.trim().to_string() };
    RenderHtml("unsafe", state.template_engine, trimmed)
}

pub async fn day14_safe(State(state): State<AppState>, Json(html_content): Json<HtmlContent>) -> impl IntoResponse {
    info!("Get safe called with content {:?}.", html_content);
    let trimmed = HtmlContent { content: html_content.content.trim().to_string() };
    RenderHtml("safe", state.template_engine, trimmed)
}