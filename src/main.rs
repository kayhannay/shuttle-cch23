use std::num::ParseIntError;

use axum::{Router, routing::get};
use axum::extract::Path;
use axum::http::StatusCode;
use tracing::log::info;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn error_500() -> Result<String, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

// async fn get_day1(Path((num1, num2)): Path<(i32, i32)>) -> String {
//     format!("{}", (num1 ^ num2).pow(3))
// }

async fn day1_get(Path(path): Path<String>) -> Result<String, StatusCode> {
    let nums: Result<Vec<i32>, ParseIntError> = path.split_terminator("/").map(|x| {
        let parsed_x = x.parse::<i32>()?;
        Ok(parsed_x)
    }).collect();
    match nums {
        Ok(nums) => {
            info!("Got nums: {:?}", nums.len());
            if nums.len() > 20 {
                return Err(StatusCode::URI_TOO_LONG);
            }
            let result = nums.iter().fold(0, |acc, x| acc ^ x).pow(3);
            Ok(format!("{}", result))
        },
        Err(_) => Err(StatusCode::BAD_REQUEST)
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error_500))
        .route("/1/*path", get(day1_get));

    Ok(router.into())
}
