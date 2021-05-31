
#[derive(Debug, serde::Deserialize, Default, Clone)]
pub struct Service {
    pub host: String,
    pub port: u16,
}
