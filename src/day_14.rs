use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use axum::routing::{post};
use axum_template::engine::Engine;
use axum_template::RenderHtml;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::{AppEngine};

#[derive(Clone)]
struct Day14State {
    template_engine: AppEngine
}

pub fn router() -> axum::Router {
    info!("Initializing template engine.");
    let mut hbs = Handlebars::new();
    hbs.register_template_file("unsafe", "./templates/unsafe.hbs").unwrap();
    hbs.register_template_file("safe", "./templates/safe.hbs").unwrap();

    info!("Initializing state.");
    let shared_state = Day14State {
        template_engine: Engine::from(hbs)
    };

    axum::Router::new()
        .route("/unsafe", post(day14_unsafe))
        .route("/safe", post(day14_safe))
        .with_state(shared_state)
}


#[derive(Serialize, Deserialize, Clone, Debug)]
struct HtmlContent {
    pub content: String,
}

async fn day14_unsafe(State(state): State<Day14State>, Json(html_content): Json<HtmlContent>) -> impl IntoResponse {
    info!("Get unsafe called with content {:?}.", html_content);
    let trimmed = HtmlContent { content: html_content.content.trim().to_string() };
    RenderHtml("unsafe", state.template_engine, trimmed)
}

async fn day14_safe(State(state): State<Day14State>, Json(html_content): Json<HtmlContent>) -> impl IntoResponse {
    info!("Get safe called with content {:?}.", html_content);
    let trimmed = HtmlContent { content: html_content.content.trim().to_string() };
    RenderHtml("safe", state.template_engine, trimmed)
}