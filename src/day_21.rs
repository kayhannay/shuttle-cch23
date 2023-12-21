use std::fmt::Binary;
use axum::extract::Path;
use axum::http::StatusCode;
use tracing::info;

pub fn router() -> axum::Router {
    let archives = axum::Router::new()
        .route("/coords/:binary", axum::routing::get(day21_coords));

    axum::Router::new().nest("/", archives)
}

#[derive(serde::Deserialize)]
struct Params {
    binary: String,
}

async fn day21_coords(params: Path<Params>) -> Result<String, StatusCode>{
    info!("Coords called.");
    let s2_cell_id = u64::from_str_radix(&params.binary, 2).unwrap();
    info!("S2 cell ID: {}", s2_cell_id);


    //let row = u8::from_str_radix(&binary[..7], 2).unwrap();
    //let col = u8::from_str_radix(&binary[7..], 2).unwrap();
    //Ok(format!("{} {}", row, col))
    Ok("".into())
}