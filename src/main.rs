use axum::{Router, routing::get};
use axum::routing::post;

use day_01::day01_get;
use day_minus1::error_500;
use day_minus1::hello_world;
use day_04::day04_post;
use day_04::day04_post_contest;
use day_06::day06_post;
use crate::day_07::{day07_get, day07_get_task2};

mod day_minus1;
mod day_01;
mod day_04;
mod day_06;
mod day_07;

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
        .route("/7/decode", get(day07_get))
        .route("/7/bake", get(day07_get_task2))
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

    #[tokio::test]
    async fn test_day07() {
        let app = init_app();
        let response = app
            .oneshot(Request::builder().uri("/7/decode").header("cookie", "recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==").body(Body::empty()).unwrap())
            .await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = body::to_bytes(response.into_body()).await.unwrap();
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
        let body = body::to_bytes(response.into_body()).await.unwrap();
        let body_string = std::str::from_utf8( &body).unwrap();
        assert_eq!(body_string, "{\"flour\":100,\"chocolate chips\":20}");
    }
}
