use std::fmt;
use std::str::FromStr;
use axum::extract::{Query};
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{post};
use serde::{de, Deserialize, Deserializer};
use tracing::info;

pub fn router() -> Router {
    Router::new().route("/", post(day05_slice))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    offset: Option<usize>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    limit: Option<usize>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    split: Option<usize>,
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

async fn day05_slice(params: Query<Params>, Json(strings): Json<Vec<String>>) -> Result<String, StatusCode> {
    if strings.is_empty() {
        return Ok("[]".into());
    }
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(strings.len());
    let split = params.split;
    info!("Slice called with offset: {}, limit: {}, split: {:?} for {:?}", offset, limit, split, &strings);
    let mut end = offset + limit;
    if end > strings.len() {
        end = strings.len();
    }
    let result = &strings[offset..end];
    if split.is_some() {
        let split = split.unwrap();
        let mut split_result = Vec::new();
        let mut index = 0;
        while index < result.len() {
            let mut end = index + split;
            if end > result.len() {
                end = result.len();
            }
            split_result.push(Vec::from(&result[index..end]));
            index += split;
        }
        return Ok(serde_json::to_string(&split_result).unwrap());
    }
    Ok(serde_json::to_string(&result.to_vec()).unwrap())
}