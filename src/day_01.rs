use std::num::ParseIntError;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use tracing::log::info;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/*nums", get(day01_get))
}

async fn day01_get(Path(path): Path<String>) -> Result<String, StatusCode> {
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

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_day01_get() {
        assert_eq!(super::day01_get(axum::extract::Path("10/".to_string())).await, Ok("1000".to_string()));
        assert_eq!(super::day01_get(axum::extract::Path("4/5/8/10".to_string())).await, Ok("27".to_string()));
    }

    #[tokio::test]
    async fn test_day01_get_not_parseable() {
        assert_eq!(super::day01_get(axum::extract::Path("2/a/3/".to_string())).await, Err(axum::http::StatusCode::BAD_REQUEST));
    }

    #[tokio::test]
    async fn test_day01_get_max_length() {
        assert_eq!(super::day01_get(axum::extract::Path("1/2/3/4/5/6/7/8/9/0/1/2/3/4/5/6/7/8/9/0".to_string())).await, Ok("0".to_string()));
        assert_eq!(super::day01_get(axum::extract::Path("1/2/3/4/5/6/7/8/9/0/1/2/3/4/5/6/7/8/9/0/1".to_string())).await, Err(axum::http::StatusCode::URI_TOO_LONG));
    }
}