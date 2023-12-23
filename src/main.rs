use axum::Router;
use axum_template::engine::Engine;
use handlebars::Handlebars;
use sqlx::PgPool;
use tracing::info;

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
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_05;
mod day_22;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    Ok(init_app_with_db(pool).await?.into())
}

type AppEngine = Engine<Handlebars<'static>>;

async fn init_app_with_db(pool: PgPool) -> Result<Router, shuttle_runtime::Error> {
    info!("Migrating database.");
    sqlx::migrate!()
        .run(&pool)
        .await.map_err(|e| shuttle_runtime::CustomError::new(e))?;

    init_app(Some(pool)).await
}
async fn init_app(pool: Option<PgPool>) -> Result<Router, shuttle_runtime::Error> {

    info!("Initializing router.");
    Ok(Router::new()
        .nest("/", day_minus1::router())
        .nest("/1", day_01::router())
        .nest("/4", day_04::router())
        .nest("/5", day_05::router())
        .nest("/6", day_06::router())
        .nest("/7", day_07::router())
        .nest("/8", day_08::router())
        .nest("/11", day_11::router())
        .nest("/12", day_12::router())
        .nest("/13", day_13::router(pool.clone()))
        .nest("/14", day_14::router())
        .nest("/15", day_15::router())
        .nest("/18", day_18::router(pool.clone()))
        .nest("/19", day_19::router())
        .nest("/20", day_20::router())
        .nest("/21", day_21::router())
        .nest("/22", day_22::router()))

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
        let app = init_app(None);
        let response = app.await.unwrap()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_string = std::str::from_utf8( &body).unwrap();
        assert_eq!(body_string, "Hello, world!");
    }

    #[tokio::test]
    async fn test_day07() {
        let app = init_app(None).await.unwrap();
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
        let app = init_app(None).await.unwrap();
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
