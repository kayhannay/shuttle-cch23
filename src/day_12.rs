use std::str::FromStr;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::{Datelike, DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;
use ulid::Ulid;
use uuid::Uuid;
use crate::AppState;

pub async fn day12_save(State(state): State<AppState>, Path(text): Path<String>) -> Result<(),StatusCode> {
    let mut texts = state.day12.lock().map_err(|_| StatusCode::BAD_REQUEST)?;
    let now = chrono::offset::Utc::now();
    info!("Got text: {} and store it with time {}", text, now);
    texts.insert(text.clone(), now);
    Ok(())
}

pub async fn day12_load(State(state): State<AppState>, Path(text): Path<String>) -> Result<String, StatusCode> {
    info!("Load text: {}", text);
    match state.day12.lock().map_err(|_| StatusCode::BAD_REQUEST)?.get(&text) {
        Some(date) => Ok((chrono::offset::Utc::now()-date).num_seconds().to_string()),
        None => Err(StatusCode::NOT_FOUND)
    }
}

pub async fn day12_ulids(Json(ulids): Json<Vec<String>>) -> Result<Json<Vec<String>>, StatusCode> {
    info!("Got ulids: {:?}", ulids);
    let mut uuids: Vec<String> = Vec::new();
    for ulid_str in ulids.iter() {
        let ulid = Ulid::from_str(&ulid_str).map_err(|_| StatusCode::BAD_REQUEST)?;
        let uuid = Uuid::from_bytes(ulid.to_bytes());
        uuids.push(uuid.to_string());
    }
    uuids.reverse();
    info!("Converted uuids: {:?}", uuids);
    Ok(Json(uuids))
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct UlidCriteria {
    #[serde(rename = "christmas eve")]
    christmas_eve: i32,
    weekday: i32,
    #[serde(rename = "in the future")]
    future: i32,
    #[serde(rename = "LSB is 1")]
    lsb_is_1: i32,
}

pub async fn day12_ulids_weekday(Path(weekday): Path<u32>, Json(ulids): Json<Vec<String>>) -> Result<Json<UlidCriteria>, StatusCode> {
    info!("Got ulids: {:?} and weekday {}", ulids, weekday);
    let mut criterias = UlidCriteria {
            christmas_eve: 0,
            weekday: 0,
            future: 0,
            lsb_is_1: 0,
        };
    for ulid_str in ulids.iter() {
        let ulid = Ulid::from_str(&ulid_str).map_err(|_| StatusCode::BAD_REQUEST)?;
        let datetime: DateTime<Utc> = ulid.datetime().into();
        let now = Utc::now();
        if datetime.weekday().num_days_from_monday() == weekday {
            criterias.weekday += 1;
        }
        if datetime.month() == 12 && datetime.day() == 24 {
            criterias.christmas_eve += 1;
        }
        if datetime > now {
            criterias.future += 1;
        }
        if ulid.to_bytes()[15] & 1 == 1 {
            criterias.lsb_is_1 += 1;
        }
    }
    Ok(Json(criterias))
}

#[cfg(test)]
mod tests {
    use axum::extract::Path;

    #[tokio::test]
    async fn test_day12_ulids() {
        let uuid_result = super::day12_ulids(axum::Json(vec![
            "01BJQ0E1C3Z56ABCD0E11HYX4M".to_string(),
            "01BJQ0E1C3Z56ABCD0E11HYX5N".to_string(),
            "01BJQ0E1C3Z56ABCD0E11HYX6Q".to_string(),
            "01BJQ0E1C3Z56ABCD0E11HYX7R".to_string(),
            "01BJQ0E1C3Z56ABCD0E11HYX8P".to_string(),
        ])).await;
        assert!(uuid_result.is_ok());
        let uuids = uuid_result.unwrap().0;
        assert_eq!(uuids, vec![
            "015cae07-0583-f94c-a5b1-a070431f7516",
            "015cae07-0583-f94c-a5b1-a070431f74f8",
            "015cae07-0583-f94c-a5b1-a070431f74d7",
            "015cae07-0583-f94c-a5b1-a070431f74b5",
            "015cae07-0583-f94c-a5b1-a070431f7494"
        ]);
    }

    #[tokio::test]
    async fn test_day12_ulids_weekday() {
        let criteria_result = super::day12_ulids_weekday(Path(5), axum::Json(vec![
            "00WEGGF0G0J5HEYXS3D7RWZGV8".to_string(),
            "76EP4G39R8JD1N8AQNYDVJBRCF".to_string(),
            "018CJ7KMG0051CDCS3B7BFJ3AK".to_string(),
            "00Y986KPG0AMGB78RD45E9109K".to_string(),
            "010451HTG0NYWMPWCEXG6AJ8F2".to_string(),
            "01HH9SJEG0KY16H81S3N1BMXM4".to_string(),
            "01HH9SJEG0P9M22Z9VGHH9C8CX".to_string(),
            "017F8YY0G0NQA16HHC2QT5JD6X".to_string(),
            "03QCPC7P003V1NND3B3QJW72QJ".to_string()
        ])).await;
        assert!(criteria_result.is_ok());
        let critaria = criteria_result.unwrap().0;
        assert_eq!(critaria, super::UlidCriteria {
            christmas_eve: 3,
            weekday: 1,
            future: 2,
            lsb_is_1: 5,
        });
    }

}