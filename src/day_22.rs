use std::collections::HashMap;
use std::io::Read;
use axum::http::StatusCode;
use rust_3d::Point3D;
use tracing::{error, info};

pub fn router() -> axum::Router {
    let archives = axum::Router::new()
        .route("/integers", axum::routing::post(day22_integers))
        .route("/rocket", axum::routing::post(day22_stars));

    axum::Router::new().nest("/", archives)
}

async fn day22_stars(data: String) -> Result<String, StatusCode> {
    info!("Stars called.");

    let mut lines = data.lines();
    let number_stars = lines
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let stars = (0..number_stars)
        .map(|_| {
            let line = lines.next().unwrap();
            let mut splitted = line.split_whitespace();
            Point3D::new(splitted.next().unwrap().parse::<f64>().unwrap(), splitted.next().unwrap().parse::<f64>().unwrap(), splitted.next().unwrap().parse::<f64>().unwrap())
        })
        .collect::<Vec<_>>();

    let number_portals = lines
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mut portal_paths: HashMap<u32,Vec<u32>> = HashMap::new();
    (0..number_portals)
        .map(|_| {
            let line = lines.next().unwrap();
            let mut splitted = line.split_whitespace();
            let source = splitted.next().unwrap().parse::<u32>().unwrap();
            let destination = splitted.next().unwrap().parse::<u32>().unwrap();
            (source, destination)
        })
        .for_each(|(source, destination)| {
            if portal_paths.contains_key(&source) {
                let mut path = portal_paths.get_mut(&source).unwrap();
                path.push(destination);
                info!("Portal path added: {} -> {:?}", source, path);
            } else {
                portal_paths.insert(source, vec![destination]);
                info!("Portal path added: {} -> {}", source, destination);
            }
        });

    let Some(path) = pathfinding::directed::bfs::bfs(
        &0,
        |p| portal_paths.get(p).cloned().unwrap_or_default(),
        |p| *p == number_stars - 1
    ) else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let distance = path
        .windows(2)
        .map(|w| rust_3d::dist_3d(stars.get(w[0] as usize).unwrap(), stars.get(w[1] as usize).unwrap()) as f32)
        .sum::<f32>();

    info!("Jumps: {}", path.len() - 1);
    info!("Distance: {:.3}", distance as f32);

    Ok(format!("{} {distance:.3}", path.len() - 1).to_string())
}

async fn day22_integers(data: String) -> Result<String, StatusCode> {
    info!("Integers called.");
    let mut parsed_data: HashMap<u64,u32> = HashMap::new();
    data.lines().for_each(|integer| {
        let integer = integer.parse::<u64>().unwrap();
        info!("Integer: {}", integer);
        if parsed_data.contains_key(&integer) {
            parsed_data.insert(integer, parsed_data.get(&integer).unwrap() + 1);
        } else {
            parsed_data.insert(integer, 1);
        }
    });
    let single_int = *parsed_data.iter().filter(|(_, count)| **count == 1).map(|(integer, _)| *integer).collect::<Vec<_>>().first().unwrap();
    let presents = "🎁".repeat(single_int as usize);
    Ok(presents)
}