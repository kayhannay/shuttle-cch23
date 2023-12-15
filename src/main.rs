use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::Router;
use axum::routing::get;
use axum::routing::post;
use axum_template::engine::Engine;
use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use sqlx::PgPool;
use tower_http::services::ServeDir;
use tracing::info;

use day_01::day01_get;
use day_04::day04_post;
use day_04::day04_post_contest;
use day_06::day06_post;
use day_07::{day07_get, day07_get_task2};
use day_08::day08_get;
use day_minus1::error_500;
use day_minus1::hello_world;
use crate::day_08::day08_get_drop;
use crate::day_12::{day12_load, day12_save, day12_ulids, day12_ulids_weekday};
use crate::day_13::{day13_insert_orders, day13_popular_orders, day13_reset, day13_sql, day13_total_orders};
use crate::day_14::{day14_safe, day14_unsafe};
use crate::day_15::{day15_game, day15_password};

mod day_minus1;
mod day_01;
mod day_04;
mod day_06;
mod day_07;
mod day_08;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    Ok(init_app(pool).await?.into())
}

type AppEngine = Engine<Handlebars<'static>>;

#[derive(Clone)]
struct AppState {
    day12: Arc<Mutex<HashMap<String, DateTime<Utc>>>>,
    db_pool: PgPool,
    template_engine: AppEngine,
}

async fn init_app(pool: PgPool) -> Result<Router, shuttle_runtime::Error> {

    info!("Migrating database.");
    sqlx::migrate!()
        .run(&pool)
        .await.map_err(|e| shuttle_runtime::CustomError::new(e))?;

    info!("Initializing template engine.");
    let mut hbs = Handlebars::new();
    hbs.register_template_file("unsafe", "./templates/unsafe.hbs").unwrap();
    hbs.register_template_file("safe", "./templates/safe.hbs").unwrap();

    info!("Initializing state.");
    let shared_state = AppState {
        day12: Arc::new(Mutex::new(HashMap::new())),
        db_pool: pool,
        template_engine: Engine::from(hbs),
    };

    info!("Initializing router.");
    Ok(Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error_500))
        .route("/1/*nums", get(day01_get))
        .route("/4/strength", post(day04_post))
        .route("/4/contest", post(day04_post_contest))
        .route("/6", post(day06_post))
        .route("/7/decode", get(day07_get))
        .route("/7/bake", get(day07_get_task2))
        .route("/8/weight/:id", get(day08_get))
        .route("/8/drop/:id", get(day08_get_drop))
        .nest_service("/11/assets/", ServeDir::new("assets"))
        .route("/11/red_pixels", post(day_11::day11_post))
        .route("/12/save/:text", post(day12_save))
        .route("/12/load/:text", get(day12_load))
        .route("/12/ulids", post(day12_ulids))
        .route("/12/ulids/:weekday", post(day12_ulids_weekday))
        .route("/13/sql", get(day13_sql))
        .route("/13/reset", post(day13_reset))
        .route("/13/orders", post(day13_insert_orders))
        .route("/13/orders/total", get(day13_total_orders))
        .route("/13/orders/popular", get(day13_popular_orders))
        .route("/14/unsafe", post(day14_unsafe))
        .route("/14/safe", post(day14_safe))
        .route("/15/nice", post(day15_password))
        .route("/15/game", post(day15_game))
        .with_state(shared_state))
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::body::to_bytes;
    use tower::util::ServiceExt;

    use crate::init_app;

    #[tokio::test]
    async fn test_app() {
        let app = init_app();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_string = std::str::from_utf8( &body).unwrap();
        assert_eq!(body_string, "Hello, world!");
    }

    #[tokio::test]
    async fn test_day07() {
        let app = init_app();
        let response = app
            .oneshot(Request::builder().uri("/7/decode").header("cookie", "recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==").body(Body::empty()).unwrap())
            .await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_string = std::str::from_utf8( &body).unwrap();
        assert_eq!(body_string, "{\"flour\":100,\"chocolate chips\":20}");
    }

    #[tokio::test]
    async fn test_day07_bake() {
        let app = init_app();
        let response = app
            .oneshot(Request::builder().uri("/7/bake").header("cookie", "recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319").body(Body::empty()).unwrap())
            .await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_string  = std::str::from_utf8(&body).unwrap();
        assert!(body_string.contains("cookies\":4"));
        assert!(body_string.contains("butter\":2002"));
        assert!(body_string.contains("sugar\":307"));
        assert!(body_string.contains("flour\":5"));
        assert!(body_string.contains("baking powder\":825"));
        assert!(body_string.contains("chocolate chips\":257"));
    }
}
