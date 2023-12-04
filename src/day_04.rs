use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;

pub async fn day04_post(Json(reindeers): Json<Vec<Reindeer>>) -> Result<String, StatusCode> {
    info!("Got reindeers: {:?}", reindeers);
    let strength: i32 = reindeers.iter().map(|reindeer| reindeer.strength).sum();
    Ok(format!("{}", strength))
}

pub async fn day04_post_contest(Json(reindeers): Json<Vec<ContestReindeer>>) -> Result<Json<ContestResult>, StatusCode> {
    info!("Got reindeers: {:?}", reindeers);
    let fastest: &ContestReindeer = reindeers
        .iter()
        .max_by(|reindeer1, reindeer2| reindeer1.speed.partial_cmp(&reindeer2.speed).unwrap())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let tallest: &ContestReindeer = reindeers
        .iter()
        .max_by_key(|reindeer| reindeer.height)
        .ok_or(StatusCode::BAD_REQUEST)?;
    let magician: &ContestReindeer = reindeers
        .iter()
        .max_by_key(|reindeer| reindeer.snow_magic_power)
        .ok_or(StatusCode::BAD_REQUEST)?;
    let consumer: &ContestReindeer = reindeers
        .iter()
        .max_by_key(|reindeer| reindeer.candies_eaten_yesterday)
        .ok_or(StatusCode::BAD_REQUEST)?;

    Ok(Json(ContestResult {
        fastest: format!("Speeding past the finish line with a strength of {} is {}", fastest.strength, fastest.name),
        tallest: format!("{} is standing tall with his {} cm wide antlers", tallest.name, tallest.antler_width),
        magician: format!("{} could blast you away with a snow magic power of {}", magician.name, magician.snow_magic_power),
        consumer: format!("{} ate lots of candies, but also some {}", consumer.name, consumer.favorite_food),
    }))
}

#[derive(Deserialize, Debug)]
pub struct Reindeer {
    #[allow(unused)]
    name: String,
    strength: i32,
}

#[derive(Deserialize, Debug)]
pub struct ContestReindeer {
    name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct ContestResult {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use axum::Json;

    use crate::day_04::ContestResult;

    #[tokio::test]
    async fn test_day04_post() {
        assert_eq!(super::day04_post(axum::Json(vec![
            super::Reindeer { name: "Rudolph".to_string(), strength: 10 },
            super::Reindeer { name: "Dasher".to_string(), strength: 20 },
            super::Reindeer { name: "Dancer".to_string(), strength: 30 },
            super::Reindeer { name: "Prancer".to_string(), strength: 40 },
            super::Reindeer { name: "Vixen".to_string(), strength: 50 },
            super::Reindeer { name: "Comet".to_string(), strength: 60 },
            super::Reindeer { name: "Cupid".to_string(), strength: 70 },
            super::Reindeer { name: "Donner".to_string(), strength: 80 },
            super::Reindeer { name: "Blitzen".to_string(), strength: 90 },
        ])).await, Ok("450".to_string()));
    }

    #[tokio::test]
    async fn test_day04_post_contest() {
        let result: Json<ContestResult> = super::day04_post_contest(axum::Json(vec![
            super::ContestReindeer {
                name: "Dasher".to_string(),
                strength: 5,
                speed: 50.4,
                height: 80,
                antler_width: 36,
                snow_magic_power: 9001,
                favorite_food: "hay".to_string(),
                candies_eaten_yesterday: 2,
            },
            super::ContestReindeer {
                name: "Dancer".to_string(),
                strength: 6,
                speed: 48.2,
                height: 65,
                antler_width: 37,
                snow_magic_power: 4004,
                favorite_food: "grass".to_string(),
                candies_eaten_yesterday: 5,
            }])).await.expect("Should be ok");

        let expected = ContestResult {
            fastest: "Speeding past the finish line with a strength of 5 is Dasher".to_string(),
            tallest: "Dasher is standing tall with his 36 cm wide antlers".to_string(),
            magician: "Dasher could blast you away with a snow magic power of 9001".to_string(),
            consumer: "Dancer ate lots of candies, but also some grass".to_string(),
        };

        let result_object = result.deref();
        assert_eq!(result_object, &expected);
    }
}