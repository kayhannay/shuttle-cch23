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

async fn day22_stars(integers: String) -> Result<String, StatusCode> {
    info!("Stars called.");
    let mut line_number = 1;
    let mut number_stars = 0;
    let mut stars: HashMap<u32, (i32, i32, i32)> = HashMap::new();
    let mut number_portals = 0;
    let mut portal_paths: HashMap<u32, Vec<u32>> = HashMap::new();
    integers.lines().for_each(|line| {
        info!("Line {}: {}", line_number, line);
        if line_number == 1 {
            number_stars = line.trim().parse::<u32>().expect(&format!("Could not parse {}", line));
        } else if line_number > 1 && line_number <= number_stars + 1 {
            let parts: Vec<String> = line.trim().split(" ").map(|text|text.to_string()).collect();
            //let point = Point3D::new(parts[0].parse::<i32>().expect(&format!("Could not parse {}", parts[0])) as f64, parts[1].parse::<i32>().expect(&format!("Could not parse {}", parts[1])) as f64, parts[2].parse::<i32>().expect(&format!("Could not parse {}", parts[2])) as f64);
            let point: (i32, i32, i32) = (parts[0].parse::<i32>().expect(&format!("Could not parse {}", parts[0])),
                         parts[1].parse::<i32>().expect(&format!("Could not parse {}", parts[1])),
                         parts[2].parse::<i32>().expect(&format!("Could not parse {}", parts[2])));
            stars.insert(line_number - 2, point);
        } else if !stars.is_empty() && line_number == number_stars + 2 {
            number_portals = line.trim().parse::<u32>().expect(&format!("Could not parse {}", line));
        } else if !stars.is_empty() && line_number > number_stars + 2 {
            let parts: Vec<String> = line.trim().split(" ").map(|text|text.to_string()).collect();
            let source: u32 = parts[0].parse().expect(&format!("Could not parse {}, line {}", parts[0], line));
            let destination: u32 = parts[1].parse().expect(&format!("Could not parse {}, line {}", parts[1], line));
            if portal_paths.contains_key(&0) && portal_paths.get(&0).unwrap().contains(&(number_stars - 1)) {
                error!("Perfect route is already there.");
            } else if portal_paths.contains_key(&source) {
                let mut path = portal_paths.get_mut(&source).unwrap();
                path.push(destination);
                info!("Portal path added: {} -> {:?}", parts[0], path);
            } else {
                portal_paths.insert(source, vec![destination]);
                info!("Portal path added: {} -> {}", parts[0], parts[1]);
            }
        } else {
            error!("Invalid line number: {}", line_number);
        }
        line_number += 1;
    });

    if stars.len() != number_stars as usize {
        error!("Number of stars does not match: {} != {}", stars.len(), number_stars);
        return Err(StatusCode::BAD_REQUEST);
    }
    if portal_paths.is_empty() {
        error!("Number of portal paths does not match: {} != {}", portal_paths.len(), number_portals);
        return Err(StatusCode::BAD_REQUEST);
    }

    let Some(path) = pathfinding::directed::bfs::bfs(
        &0,
        |p| portal_paths.get(p).cloned().unwrap_or_default(),
        |p| *p == number_stars - 1
    ) else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let distance = path
        .windows(2)
        .map(|w| calc_distance(stars.get(&w[0]).unwrap(), stars.get(&w[1]).unwrap()))
        .sum::<f32>();

    info!("Jumps: {}", path.len() - 1);
    info!("Distance: {:.3}", distance as f32);

    Ok(format!("{} {distance:.3}", path.len() - 1).to_string())
}

fn calc_distance(star1: &(i32, i32, i32), star2: &(i32, i32, i32)) -> f32 {
    let dist = (star2.0 - star1.0).pow(2) + (star2.1 - star1.1).pow(2) + (star2.2 - star1.2).pow(2);
    (dist as f32).sqrt()
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
    let presents = (0u64..single_int).map(|_| "🎁".to_string()).collect::<Vec<String>>().join("");
    Ok(presents)
}