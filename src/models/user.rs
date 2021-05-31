

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Claims {
    pub id: u32,
    pub email: String,
    pub active: bool,
}