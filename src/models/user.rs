#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Claims {
    pub id: u32,
    pub email: String,
    pub active: bool,
    pub exp: u64,
}
