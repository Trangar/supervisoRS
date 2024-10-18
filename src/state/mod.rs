mod preset;
mod theme;

pub use preset::*;
pub use theme::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    location: Point,
    size: Size,
    theme: Theme,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Size {
    width: f32,
    height: f32,
}
