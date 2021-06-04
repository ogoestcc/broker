use validator::Validate;


#[path = "collaborative_filtering.rs"]
mod cf;

#[path = "content_based.rs"]
mod cb;

#[derive(Debug, Validate, serde::Deserialize, Clone)]
pub struct Request {
    #[validate(range(min = 1))]
    #[serde(default)]
    n: Option<u32>,
}

pub use cf::collaborative_filtering;
pub use cb::content_based;
