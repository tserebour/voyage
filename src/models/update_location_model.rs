use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateDriverLocation {
    pub id: i32,
    pub current_latitude: f64,
    pub current_longitude: f64,
}