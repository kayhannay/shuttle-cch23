use std::fmt::Binary;
use axum::extract::Path;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::info;

pub fn router() -> axum::Router {
    let archives = axum::Router::new()
        .route("/coords/:binary", axum::routing::get(day21_coords))
        .route("/country/:binary", axum::routing::get(day21_country));

    axum::Router::new().nest("/", archives)
}

#[derive(serde::Deserialize)]
struct Params {
    binary: String,
}

async fn day21_coords(params: Path<Params>) -> Result<String, StatusCode>{
    info!("Coords called with {}.", &params.binary);
    let (lat, lon) = convert_cell_to_coordinates(&params.binary);
    let latitude = format!("{}°{}'{:.3}''{}",
                           lat.abs() as u8,
                           (lat.fract() * 60.0).abs() as u8,
                           ((lat.fract() * 60.0).fract() * 60.0).abs(),
                           if lat.is_sign_positive() { "N" } else { "S" });
    let longitude = format!("{}°{}'{:.3}''{}",
                           lon.abs() as u8,
                           (lon.fract() * 60.0).abs() as u8,
                           ((lon.fract() * 60.0).fract() * 60.0).abs(),
                           if lon.is_sign_positive() { "E" } else { "W" });

    Ok(format!("{} {}", latitude, longitude).into())
}

#[derive(Deserialize, Serialize)]
struct Country {
    address: Address,
}

#[derive(Deserialize, Serialize)]
struct Address {
    country_code: String,
    country: String,
}

async fn day21_country(params: Path<Params>) -> Result<String, StatusCode> {
    info!("Country called with {}.", &params.binary);
    let (lat, lon) = convert_cell_to_coordinates(&params.binary);
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:85.0) Gecko/20100101 Firefox/85.0")
        .build().unwrap();
    let osm_response = client.get(format!("https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=json", lat, lon))
        .header("accept-language", "en-US,en;q=0.9,de;q=0.8,fr;q=0.7")
        .send()
        .await;
    let country = osm_response.unwrap().json::<Country>().await.unwrap().address.country;
    info!("Country: {}", country);
    Ok(format!("{}", country).into())
}

fn convert_cell_to_coordinates(call_id_string: &String) -> (f64, f64) {
    let s2_cell_id = u64::from_str_radix(call_id_string, 2).unwrap();
    info!("S2 cell ID: {}", s2_cell_id);
    let cell_id = s2::cellid::CellID(s2_cell_id);
    let cell = s2::cell::Cell::from(cell_id);
    let center = cell.center();
    let lat = center.latitude().deg();
    let lon = center.longitude().deg();
    (lat, lon)
}