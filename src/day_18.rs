use std::collections::HashMap;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
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

pub async fn day18_reset(State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    info!("Reset SQL called.");
    let pool = state.db_pool.unwrap();
    sqlx::query("DROP TABLE IF EXISTS orders")
        .execute(&pool)
        .await.map_err(|_| StatusCode::BAD_REQUEST)?;
    sqlx::query("DROP TABLE IF EXISTS regions")
        .execute(&pool)
        .await.map_err(|_| StatusCode::BAD_REQUEST)?;
    sqlx::query("CREATE TABLE orders (
            id INT PRIMARY KEY,
            region_id INT,
            gift_name VARCHAR(50),
            quantity INT
        )")
        .execute(&pool)
        .await.map_err(|_| StatusCode::BAD_REQUEST)?;
    match sqlx::query("CREATE TABLE regions (
            id INT PRIMARY KEY,
            name VARCHAR(50)
        )")
        .execute(&pool)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn day18_insert_orders(State(state): State<AppState>, Json(orders): Json<Vec<Order>>) -> Result<StatusCode,StatusCode> {
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

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Region {
    pub id: i32,
    pub name: String,
}

pub async fn day18_insert_regions(State(state): State<AppState>, Json(regions): Json<Vec<Region>>) -> Result<StatusCode,StatusCode> {
    info!("Insert regions: {:?}", regions);
    let pool = state.db_pool.unwrap();
    for region in regions {
        let _ = sqlx::query("INSERT INTO regions (id, name) VALUES ($1, $2)")
            .bind(region.id)
            .bind(&region.name)
            .execute(&pool)
            .await.map_err(|_| StatusCode::BAD_REQUEST)?;

    }
    Ok(StatusCode::OK)
}


#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OrderPerRegionCount {
    pub region: String,
    pub total: i64,
}

pub async fn day18_total_orders_per_region(State(state): State<AppState>) -> Result<Json<Vec<OrderPerRegionCount>>, StatusCode> {
    info!("Total orders per region called.");
    let pool = state.db_pool.unwrap();
    let rows = sqlx::query("SELECT regions.name, SUM(orders.quantity) FROM orders INNER JOIN regions ON orders.region_id = regions.id GROUP BY regions.name")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let totals: Result<Vec<OrderPerRegionCount>, StatusCode> = rows.iter().map(|row| {
        let region: String = row.try_get(0).map_err(|_| StatusCode::BAD_REQUEST)?;
        let total: i64 = row.try_get(1).map_err(|_| StatusCode::BAD_REQUEST)?;
        info!("Total orders for region {}: {}", region, total);
        Ok(OrderPerRegionCount { region, total })
    }).collect();
    match totals {
        Ok(mut totals) => {
            totals.sort();
            Ok(Json(totals))
        },
        Err(e) => Err(e),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Popular {
    pub region: String,
    pub top_gifts: Vec<String>,
}

pub async fn day18_popular_orders_per_region(State(state): State<AppState>, Path(max): Path<i32>) -> Result<Json<Vec<Popular>>,StatusCode> {
    info!("Popular orders per region called.");
    let pool = state.db_pool.unwrap();
    let popular_orders_per_region = sqlx::query("SELECT regions.name, orders.gift_name, SUM(orders.quantity) FROM orders INNER JOIN regions ON orders.region_id = regions.id GROUP BY regions.name, orders.gift_name")
        .fetch_all(&pool)
        .await.unwrap_or_else(|_| vec![]);
    info!("Popular orders per region received");
    let mut orders: HashMap<String,Vec<(String, i64)>> = HashMap::new();
    popular_orders_per_region.iter()
        .for_each(|row| {
            let region = row.get::<String, _>("name");
            let gift_name = row.get::<String, _>("gift_name");
            let quantity = row.get::<i64, _>(2);
            if orders.contains_key(&region) {
                let region_orders = orders.get_mut(&region).unwrap();
                region_orders.push((gift_name.clone(), quantity));
            } else {
                let region_orders = vec![(gift_name.clone(), quantity)];
                orders.insert(region.clone(), region_orders);
            }
        });
    let rows = sqlx::query("SELECT name FROM regions")
        .fetch_all(&pool)
        .await.unwrap_or_else(|_| vec![]);
    for row in rows {
        let region = row.get::<String, _>("name");
        if !orders.contains_key(&region) {
            orders.insert(region.clone(), vec![]);
        }
    }
    info!("Popular orders: {:?}", orders);
    let mut popular: Vec<Popular> = orders.iter()
        .map(|(region, region_orders)| {
            let mut region_orders = region_orders.clone();
            region_orders.sort_by(|a, b| {
                if a.1 == b.1 {
                    return a.0.cmp(&b.0);
                }
                b.1.cmp(&a.1)
            });
            let top_gifts: Vec<String> = region_orders.iter()
                .take(max as usize)
                .map(|(gift_name, _)| gift_name.clone())
                .collect();
            Popular { region: region.clone(), top_gifts }
        })
        .collect();
    popular.sort_by(|a, b| a.region.cmp(&b.region));
    Ok(Json(popular))
}

