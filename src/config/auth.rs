
#[derive(Debug, serde::Deserialize, Default, Clone)]
pub struct Auth {
    pub secret_key: String,
}
