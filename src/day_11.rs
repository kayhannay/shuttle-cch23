use axum::http::StatusCode;
use axum::routing::post;
use axum_extra::extract::Multipart;
use image::GenericImageView;
use tower_http::services::ServeDir;
use tracing::info;

pub fn router() -> axum::Router {
    axum::Router::new()
        .nest_service("/assets/", ServeDir::new("assets"))
        .route("/red_pixels", post(day11_post))
}

async fn day11_post(mut multipart: Multipart) -> Result<String, StatusCode> {
    let mut red_pixels = 0;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        info!("Length of `{}` is {} bytes", name, data.len());
        if name == "image" {
            let image = image::load_from_memory(&data).unwrap();
            info!("Image size: {}x{}", image.width(), image.height());
            image.pixels().for_each(|pixel| {
                let pixel_data = pixel.2;
                let red = pixel_data[0] as i32;
                let green = pixel_data[1] as i32;
                let blue = pixel_data[2] as i32;
                if red > (blue + green) {
                    red_pixels += 1;
                }
            });
            info!("Red pixels: {}", red_pixels);
        }
    }
    Ok(format!("{}", red_pixels))
}