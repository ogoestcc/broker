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
            .map_err(UsersError::from)?;

        let response = receiver.await.map_err(UsersError::from)?;

        let user = response
            .get_users()
            .get(0)
            .ok_or_else(UsersError::not_found)?;

        if user.has_active() && !user.get_active() {
            return Err(UsersError::Inactive.into());
        }

        if user.has_deleted_at() {
            return Err(UsersError::Inactive.into());
        }

        Ok(user.to_owned())
    }

    pub async fn create_user(
        &self,
        email: String,
        password: String,
    ) -> Result<users::User, ServiceError<UsersError>> {
        let mut payload = users::operations::Payload::new();
        payload.set_email(email);
        payload.set_password(password);

        let mut request = users::operations::Create::new();
        request.set_user(payload);

        let receiver = self
            .db
            .users
            .create_async(&request)
            .map_err(UsersError::from)?;

        Ok(receiver.await.map_err(UsersError::from)?)
    }
}
