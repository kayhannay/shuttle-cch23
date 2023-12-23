use std::collections::HashMap;
use std::str::Lines;
use axum::http::StatusCode;
use rust_3d::Point3D;
use tracing::{info};

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

    let stars = get_stars(&mut lines, number_stars);

    let number_portals = lines
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let portal_paths = get_portal_paths(lines, number_portals);

    let path = match get_shortest_path(number_stars, portal_paths) {
        Ok(value) => value,
        Err(value) => return Err(value),
    };

    let distance = path
        .windows(2)
        .map(|w| rust_3d::dist_3d(stars.get(w[0] as usize).unwrap(), stars.get(w[1] as usize).unwrap()) as f32)
        .sum::<f32>();

    info!("Jumps: {}", path.len() - 1);
    info!("Distance: {:.3}", distance as f32);

    Ok(format!("{} {distance:.3}", path.len() - 1).to_string())
}

fn get_shortest_path(number_stars: u32, portal_paths: HashMap<u32, Vec<u32>>) -> Result<Vec<u32>, StatusCode> {
    let Some(path) = pathfinding::directed::bfs::bfs(
        &0,
        |p| portal_paths.get(p).cloned().unwrap_or_default(),
        |p| *p == number_stars - 1
    ) else {
        return Err(StatusCode::BAD_REQUEST);
    };
    Ok(path)
}

fn get_stars(lines: &mut Lines, number_stars: u32) -> Vec<Point3D> {
    let stars = (0..number_stars)
        .map(|_| {
            let line = lines.next().unwrap();
            let mut splitted = line.split_whitespace();
            Point3D::new(splitted.next().unwrap().parse::<f64>().unwrap(), splitted.next().unwrap().parse::<f64>().unwrap(), splitted.next().unwrap().parse::<f64>().unwrap())
        })
        .collect::<Vec<_>>();
    stars
}

fn get_portal_paths(mut lines: Lines, number_portals: u32) -> HashMap<u32, Vec<u32>> {
    let mut portal_paths: HashMap<u32, Vec<u32>> = HashMap::new();
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
                let path = portal_paths.get_mut(&source).unwrap();
                path.push(destination);
                info!("Portal path added: {} -> {:?}", source, path);
            } else {
                portal_paths.insert(source, vec![destination]);
                info!("Portal path added: {} -> {}", source, destination);
            }
        });
    portal_paths
}

async fn day22_integers(data: String) -> Result<String, StatusCode> {
    info!("Integers called.");
    data.lines()
        .map(|line| usize::from_str_radix(line, 10))
        .try_fold(0, |acc, current_number| current_number.map(|x| acc ^ x))
        .map(|single_sumber| "🎁".repeat(single_sumber).to_string())
        .map_err(|_| StatusCode::BAD_REQUEST)
}