use protos::types::users;

use crate::{
    grpc::database::DatabaseService,
    resources::errors::{users::UsersError, ServiceError},
};

impl DatabaseService {
    pub async fn get_user_by_email(
        &mut self,
        email: &String,
    ) -> Result<users::User, ServiceError<UsersError>> {
        let mut r#where = users::WhereClause::default();
        r#where.set_email(email.to_owned());

        let mut req = users::get_users::Request::default();
        req.set_field_where(r#where);

        let receiver = self
            .db
            .users
            .get_users_async(&req)
            .map_err(UsersError::from)
            .map_err(ServiceError::internal)?;

        let response = receiver
            .await
            .map_err(UsersError::from)
            .map_err(ServiceError::internal)?;

        let user = response
            .get_users()
            .get(0)
            .ok_or_else(UsersError::not_found)
            .map_err(ServiceError::internal)?;

        if user.has_active() && !user.get_active() {
            Err(UsersError::Inactive).map_err(ServiceError::internal)?;
        }

        if user.has_deleted_at() {
            Err(UsersError::Inactive).map_err(ServiceError::internal)?;
        }

        Ok(user.to_owned())
    }
}
