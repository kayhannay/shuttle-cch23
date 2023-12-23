use std::collections::HashMap;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};
use sqlx::postgres::PgRow;
use tracing::info;

#[derive(Clone)]
struct Day13State {
    db_pool: Option<PgPool>
}

pub fn router(pool: Option<PgPool>) -> axum::Router {
    info!("Initializing state.");
    let shared_state = Day13State {
        db_pool: pool,
    };

    axum::Router::new()
        .route("/sql", get(day13_sql))
        .route("/reset", post(day13_reset))
        .route("/orders", post(day13_insert_orders))
        .route("/orders/total", get(day13_total_orders))
        .route("/orders/popular", get(day13_popular_orders))
        .with_state(shared_state)
}

#[derive(Serialize, FromRow, Debug)]
struct Get {
    pub id: i32,
    pub num: i32,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
struct Order {
    pub id: i32,
    pub region_id: i32,
    pub gift_name: String,
    pub quantity: i32,
}

async fn day13_sql(State(state): State<Day13State>) -> Result<String, StatusCode> {
    info!("Get SQL called.");
    let pool = state.db_pool.unwrap();
    match sqlx::query_as::<_, Get>("SELECT * FROM day13_get")
        .fetch_one(&pool)
        .await
    {
        Ok(get) => Ok(format!("{}", get.num)),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn day13_reset(State(state): State<Day13State>) -> Result<StatusCode, StatusCode> {
    info!("Reset SQL called.");
    let pool = state.db_pool.unwrap();
    sqlx::query("DROP TABLE IF EXISTS orders")
        .execute(&pool)
        .await.map_err(|_| StatusCode::BAD_REQUEST)?;
    match sqlx::query("CREATE TABLE orders (
            id INT PRIMARY KEY,
            region_id INT,
            gift_name VARCHAR(50),
            quantity INT
        )")
        .execute(&pool)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn day13_insert_orders(State(state): State<Day13State>, Json(orders): Json<Vec<Order>>) -> Result<StatusCode,StatusCode> {
    info!("Insert orders: {:?}", orders);
    let pool = state.db_pool.unwrap();
    for order in orders {
        let _ = sqlx::query("INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)")
            .bind(order.id)
            .bind(order.region_id)
            .bind(&order.gift_name)
            .bind(order.quantity)
            .execute(&pool)
            .await.map_err(|_| StatusCode::BAD_REQUEST)?;

    }
    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize, Debug)]
struct OrderCount {
    pub total: i64,
}

async fn day13_total_orders(State(state): State<Day13State>) -> Result<Json<OrderCount>, StatusCode> {
    info!("Total orders called.");
    let pool = state.db_pool.unwrap();
    let row: PgRow = sqlx::query("SELECT SUM(quantity) FROM orders")
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let total: i64 = row.try_get(0).map_err(|_| StatusCode::BAD_REQUEST)?;
    info!("Total orders: {}", total);
    Ok(Json(OrderCount { total }))
}

#[derive(Serialize, Deserialize, Debug)]
struct Popular {
    pub popular: Option<String>,
}

async fn day13_popular_orders(State(state): State<Day13State>) -> Result<Json<Popular>,StatusCode> {
    info!("Popular orders called.");
    let pool = state.db_pool.unwrap();
    let rows = sqlx::query("SELECT * FROM orders")
        .fetch_all(&pool)
        .await.unwrap_or_else(|_| vec![]);
    if rows.is_empty() {
        return Ok(Json(Popular { popular: None }))
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
    Ok(Json(Popular { popular: Some(popular.to_string()) }))
}

