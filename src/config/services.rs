#[derive(Debug, serde::Deserialize, Default, Clone)]
pub struct Service {
    pub host: String,
    pub port: u16,
}

impl ToString for Service {
    fn to_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
