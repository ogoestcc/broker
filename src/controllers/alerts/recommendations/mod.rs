mod collaborative_filtering;
#[path = "content_based.rs"]
mod cb;

mod top;

pub use top::top_alerts;
pub use cb::content_based;
