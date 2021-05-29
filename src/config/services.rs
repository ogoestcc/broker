
#[derive(Debug, serde::Deserialize)]
pub struct Service {
    pub host: String,
    pub port: u16,
}
