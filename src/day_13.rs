use std::collections::HashMap;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use tracing::info;
use crate::AppState;

#[derive(Serialize, FromRow, Debug)]
struct Get {
    pub id: i32,
    pub num: i32,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Order {
    pub id: i32,
    pub region_id: i32,
    pub gift_name: String,
    pub quantity: i32,
}

pub async fn day13_sql(State(state): State<AppState>) -> Result<String, StatusCode> {
    info!("Get SQL called.");
    match sqlx::query_as::<_, Get>("SELECT * FROM day13_get")
        .fetch_one(&state.db_pool)
        .await
    {
        Ok(get) => Ok(format!("{}", get.num)),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn day13_reset(State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    info!("Reset SQL called.");
    sqlx::query("DROP TABLE IF EXISTS orders")
        .execute(&state.db_pool)
        .await.map_err(|_| StatusCode::BAD_REQUEST)?;
    match sqlx::query("CREATE TABLE orders (
            id INT PRIMARY KEY,
            region_id INT,
            gift_name VARCHAR(50),
            quantity INT
        )")
        .execute(&state.db_pool)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn day13_insert_orders(State(state): State<AppState>, Json(orders): Json<Vec<Order>>) -> Result<StatusCode,StatusCode> {
    info!("Insert orders: {:?}", orders);
    for order in orders {
        let _ = sqlx::query("INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)")
            .bind(order.id)
            .bind(order.region_id)
            .bind(&order.gift_name)
            .bind(order.quantity)
            .execute(&state.db_pool)
            .await.map_err(|_| StatusCode::BAD_REQUEST)?;

    }
    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderCount {
    pub total: i64,
}

pub async fn day13_total_orders(State(state): State<AppState>) -> Result<Json<OrderCount>, StatusCode> {
    info!("Total orders called.");
    let row: PgRow = sqlx::query("SELECT SUM(quantity) FROM orders")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let total: i64 = row.try_get(0).map_err(|_| StatusCode::BAD_REQUEST)?;
    info!("Total orders: {}", total);
    Ok(Json(OrderCount { total }))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Popular {
    pub total: String,
}

pub async fn day13_popular_orders(State(state): State<AppState>) -> Result<String,StatusCode> {
    info!("Popular orders called.");
    let rows = sqlx::query("SELECT * FROM orders")
        .fetch_all(&state.db_pool)
        .await.unwrap_or_else(|_| vec![]);
    if rows.is_empty() {
        return StatusCode::
    }
    let mut orders: HashMap<String,i32> = HashMap::new();
    rows.iter()
        .for_each(|row| {
            let name = row.get::<String, _>("gift_name");
            let quantity = row.get::<i32, _>("quantity");
            if orders.contains_key(&name) {
                orders.insert(name.clone(), orders.get(&name).unwrap() + quantity);
            } else {
                orders.insert(name.clone(), quantity);
            }
        });
    let (popular, quantity) = orders.iter()
        .max_by_key(|order| order.1)
        .ok_or(StatusCode::BAD_REQUEST)?;
    info!("Popular order: {} with {} orders", popular, quantity);
    Ok(serde_json::to_string(&Popular { total: popular.to_string() }).unwrap())
}

