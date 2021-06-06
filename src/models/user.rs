#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Claims {
    pub id: u32,
    pub email: String,
    pub active: bool,
    pub exp: u64,
}



#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct User {
    id: u32,
    email: String,
    active: bool,
    created_at: String,
    updated_at: String,
}


impl From<protos::types::users::User> for User {
    fn from(mut proto: protos::types::users::User) -> Self {
        User {
            id: proto.get_id() as u32,
            email: proto.take_email(),
            active: proto.get_active(),
            created_at: proto.take_created_at(),
            updated_at: proto.take_updated_at(),
        }
    }
}