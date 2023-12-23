use axum::http::StatusCode;
use axum::routing::get;
use tracing::info;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error_500))
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn error_500() -> Result<String, StatusCode> {
    info!("Return error 500");
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_hello_world() {
        assert_eq!(super::hello_world().await, "Hello, world!");
    }

    #[tokio::test]
    async fn test_error_500() {
        assert_eq!(super::error_500().await, Err(StatusCode::INTERNAL_SERVER_ERROR));
    }
}