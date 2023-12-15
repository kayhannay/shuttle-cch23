use axum::extract::Path;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::info;

const GRAVITY: f32 = 9.825;

#[derive(Deserialize, Serialize, Debug)]
struct Pokemon {
    name: String,
    id: i32,
    weight: i32,
}
pub async fn day08_get(Path(id): Path<i32>) -> Result<String, StatusCode> {
    day08_get_impl("https://pokeapi.co/".to_string(), id).await
}

async fn get_pokemon(api: String, id: i32) -> Result<Pokemon, StatusCode> {
    let uri = format!("{}/api/v2/pokemon/{}", api, id);
    info!("Calling {}", uri);
    let response = reqwest::get(uri).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .json::<Pokemon>().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    match response {
        Ok(pokemon) => Ok(pokemon),
        Err(_) => Err(StatusCode::NOT_FOUND)
    }
}

async fn day08_get_impl(api: String, id: i32) -> Result<String, StatusCode> {
    let pokemon = get_pokemon(api, id).await?;
    Ok(format!("{}", pokemon.weight as f32 / 10f32))
}

pub async fn day08_get_drop(Path(id): Path<i32>) -> Result<String, StatusCode> {
    day08_get_drop_impl("https://pokeapi.co/".to_string(), id).await
}

async fn day08_get_drop_impl(api: String, id: i32) -> Result<String, StatusCode> {
    let pokemon = get_pokemon(api, id).await?;
    let speed = (2f32 * GRAVITY * 10f32).sqrt();
    info!("Speed: {}", speed);
    let impulse = speed * (pokemon.weight as f32 / 10f32);
    info!("Impulse: {}", impulse);
    Ok(format!("{}", impulse))
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};

    use crate::day_08::Pokemon;

    #[tokio::test]
    async fn test_day08_get() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v2/pokemon/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(Pokemon {
                name: "bulbasaur".to_string(),
                id: 1,
                weight: 69,
            }))
            .mount(&mock_server)
            .await;
        assert_eq!(super::day08_get_impl(mock_server.uri(), 1).await, Ok("6.9".to_string()));
    }

    #[tokio::test]
    async fn test_day08_get_drop() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v2/pokemon/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(Pokemon {
                name: "bulbasaur".to_string(),
                id: 1,
                weight: 69,
            }))
            .mount(&mock_server)
            .await;
        assert_eq!(super::day08_get_drop_impl(mock_server.uri(), 1).await,  Ok("96.72314".to_string()));
    }
}
