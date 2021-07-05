use protos::types::alerts;

use crate::{
    grpc::database::DatabaseService,
    resources::errors::{alerts::AlertsError, ServiceError},
};

impl DatabaseService {
    pub async fn get_viewed_alerts(
        &mut self,
        user_id: u32,
        favorited: Option<bool>,
    ) -> Result<Vec<alerts::Alert>, ServiceError<AlertsError>> {
        let mut r#where = alerts::WhereClause::default();

        let mut view = alerts::View::default();

        view.set_user_id(user_id as i64);
        if let Some(favorited) = favorited {
            view.set_favorited(favorited);
        }

        r#where.set_viewer(view);

        let mut req = alerts::get_alerts::Request::default();
        req.set_field_where(r#where);

        let receiver = self
            .db
            .alerts
            .get_alerts_async(&req)
            .map_err(AlertsError::from)?;

        let response = receiver.await.map_err(AlertsError::from)?;
        let alerts = response.get_alerts();

        Ok(alerts.to_owned())
    }

    pub async fn get_alerts(&mut self) -> Result<Vec<alerts::Alert>, ServiceError<AlertsError>> {
        let mut req = alerts::get_alerts::Request::default();
        req.set_field_where(Default::default());

        let receiver = self
            .db
            .alerts
            .get_alerts_async(&req)
            .map_err(AlertsError::from)?;

        let response = receiver.await.map_err(AlertsError::from)?;
        let alerts = response.get_alerts();

        Ok(alerts.to_owned())
    }
}
