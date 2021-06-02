use crate::protos as proto_def;

pub use proto_def::types::{User, UserWhereClause as WhereClause};

pub mod get_users {
    use super::proto_def::users;

    pub use users::{
        GetUsers_Metadata as Metadata, GetUsers_Request as Request, GetUsers_Response as Response,
    };
}

pub mod get_users_and_contents {
    use super::proto_def::users;

    pub use users::{
        GetUsersAndContents_Metadata as Metadata, GetUsersAndContents_Response as Response,
        GetUsersAndContents_UsersContents as UserContents, GetUsers_Request as Request,
    };
}

pub mod get_users_and_ratings {
    use super::proto_def::users;

    pub use users::{
        GetUsersAndRatings_Metadata as Metadata, GetUsersAndRatings_Request as Request,
        GetUsersAndRatings_Response as Response, GetUsersAndRatings_UsersRatings as UserRatings,
    };
}
