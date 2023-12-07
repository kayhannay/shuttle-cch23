use axum::extract::TypedHeader;
use axum::headers::Cookie;
use axum::http::StatusCode;
use axum::Json;
use lib_base64::Base64;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

pub async fn day07_get(TypedHeader(cookie): TypedHeader<Cookie>) -> Result<String,StatusCode> {
    match cookie.get("recipe") {
        Some(recipe) => {
            recipe.to_string().decode().map_err(|_| StatusCode::BAD_REQUEST)
        }
        None => Err(StatusCode::BAD_REQUEST)
    }
}

#[derive(Deserialize, Debug)]
pub struct BakeData{
    recipe: Ingredients,
    pantry: Ingredients,
}

#[derive(Serialize, Debug)]
pub struct BakeResult{
    cookies: i32,
    pantry: Ingredients,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Ingredients{
    flour: i32,
    sugar: i32,
    butter: i32,
    #[serde(rename = "baking powder")]
    baking_powder: i32,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: i32,
}

impl Ingredients {
    pub fn bake(&mut self, recipe: &Ingredients) -> bool {
        if self.flour < recipe.flour {
            return false;
        }
        if self.sugar < recipe.sugar {
            return false;
        }
        if self.butter < recipe.butter {
            return false;
        }
        if self.baking_powder < recipe.baking_powder {
            return false;
        }
        if self.chocolate_chips < recipe.chocolate_chips {
            return false;
        }
        self.flour -= recipe.flour;
        self.sugar -= recipe.sugar;
        self.butter -= recipe.butter;
        self.baking_powder -= recipe.baking_powder;
        self.chocolate_chips -= recipe.chocolate_chips;
        true
    }
}

pub async fn day07_get_task2(TypedHeader(cookie): TypedHeader<Cookie>) -> Result<Json<BakeResult>,StatusCode> {
    info!("Got cookie: {:?}", cookie);
    let data = cookie.get("recipe")
        .and_then(|data| Some(data.to_string().decode())).ok_or(StatusCode::BAD_REQUEST)?.map_err(|_| StatusCode::BAD_REQUEST);
    info!("Got data: {:?}", data);
    let bake_data: BakeData = serde_json::from_str(&data.unwrap()).map_err(|e| { error!("Could not parse data: {}", e); StatusCode::BAD_REQUEST})?;
    info!("Got bake data: {:?}", bake_data);
    let recipe = bake_data.recipe;
    let mut pantry = bake_data.pantry;
    let mut cookie_counter = 0;
    while pantry.bake(&recipe) {
        cookie_counter += 1;
    }
    Ok(Json(BakeResult {
        cookies: cookie_counter,
        pantry,
    }))
}
