use axum::{Router, routing::get};
use axum::routing::post;

use day_01::day01_get;
use day_minus1::error_500;
use day_minus1::hello_world;
use day_04::day04_post;
use day_04::day04_post_contest;
use day_06::day06_post;

mod day_minus1;
mod day_01;
mod day_04;
mod day_06;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(init_app().into())
}

fn init_app() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/500", get(error_500))
        .route("/1/:nums", get(day01_get))
        .route("/4/strength", post(day04_post))
        .route("/4/contest", post(day04_post_contest))
        .route("/6", post(day06_post))
}

#[cfg(test)]
mod tests {
    use hyper::body;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    use crate::init_app;

    #[tokio::test]
    async fn test_app() {
        let app = init_app();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = body::to_bytes(response.into_body()).await.unwrap();
        let body_string = std::str::from_utf8( &body).unwrap();
        assert_eq!(body_string, "Hello, world!");
    }
}
