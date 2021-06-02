use crate::protos as proto_def;

pub use proto_def::types::{Rating, RatingWhereClause as WhereClause};

pub mod get_ratings {
    use super::proto_def::ratings;

    pub use ratings::{
        GetRatings_Metadata as Metadata, GetRatings_Request as Request,
        GetRatings_Response as Response,
    };
}
