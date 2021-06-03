use crate::protos as proto_def;

pub use proto_def::types::{Alert, AlertWhereClause as WhereClause};

pub mod get_alerts {
    use super::proto_def::alerts;

    pub use alerts::{
        GetAlerts_Metadata as Metadata, GetAlerts_Request as Request,
        GetAlerts_Response as Response,
    };
}

pub mod get_alerts_and_ratings {
    use super::proto_def::alerts;

    pub use alerts::{
        GetAlertsAndRatings_Metadata as Metadata, GetAlertsAndRatings_Request as Request,
        GetAlertsAndRatings_Response as Response, GetAlertsAndRatings_AlertsRatings as AlertsRatings,
    };
}