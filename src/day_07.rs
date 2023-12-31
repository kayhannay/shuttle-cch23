use std::collections::HashMap;
use axum::http::StatusCode;
use axum::Json;
use axum::routing::{get};
use axum_extra::headers::Cookie;
use axum_extra::TypedHeader;
use lib_base64::Base64;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/decode", get(day07_get))
        .route("/bake", get(day07_get_task2))
}

async fn day07_get(TypedHeader(cookie): TypedHeader<Cookie>) -> Result<String,StatusCode> {
    match cookie.get("recipe") {
        Some(recipe) => {
            recipe.to_string().decode().map_err(|_| StatusCode::BAD_REQUEST)
        }
        None => Err(StatusCode::BAD_REQUEST)
    }
}

#[derive(Deserialize, Debug)]
struct BakeData{
    recipe: HashMap<String, i64>,
    pantry: HashMap<String, i64>,
}

#[derive(Serialize, Debug)]
struct BakeResult{
    cookies: i64,
    pantry: HashMap<String, i64>,
}

impl BakeData {
    fn ingredients_available(&mut self) -> bool {
        let available = self.recipe.iter()
            .filter(|(ingredient, amount)|
                self.pantry.contains_key(ingredient as &str)
                    && self.pantry.get(ingredient as &str).unwrap() >= amount)
            .count() == self.recipe.len();
        if available {
            self.recipe.iter()
                .for_each(|(ingredient, amount)| {
                let current_amount = self.pantry.get(ingredient).unwrap();
                self.pantry.insert(ingredient.to_string(), current_amount - amount);
            });
            true
        } else {
            false
        }
    }

    fn bake(&mut self) -> BakeResult {
        let mut cookie_counter = 0;
        self.recipe.retain(|_, amount| *amount > 0i64);
        info!("Recipe after cleanup: {:?}", self.recipe);
        while self.ingredients_available() {
            cookie_counter += 1;
        }
        BakeResult {
            cookies: cookie_counter,
            pantry: self.pantry.clone(),
        }
    }
}

async fn day07_get_task2(TypedHeader(cookie): TypedHeader<Cookie>) -> Result<Json<BakeResult>,StatusCode> {
    info!("Got cookie: {:?}", cookie);
    let data = cookie.get("recipe")
        .and_then(|data| Some(data.to_string().decode())).ok_or(StatusCode::BAD_REQUEST)?.map_err(|_| StatusCode::BAD_REQUEST);
    info!("Got data: {:?}", data);
    let mut bake_data: BakeData = serde_json::from_str(&data.unwrap()).map_err(|e| { error!("Could not parse data: {}", e); StatusCode::BAD_REQUEST})?;
    info!("Got bake data: {:?}", bake_data);
    let bake_result = bake_data.bake();
    info!("Bake result: {:?}", bake_result);
    Ok(Json(bake_result))
}
