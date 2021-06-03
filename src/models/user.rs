use std::time::{Duration, UNIX_EPOCH};


// use time;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Claims {
    pub id: u32,
    pub email: String,
    pub active: bool,
    pub exp: u64,
}
