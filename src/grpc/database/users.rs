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

        let receiver = self.db.users.get_users_async(&req);

        match receiver {
            Ok(receiver) => match receiver.await {
                Ok(response) => match response.get_users().get(0) {
                    Some(user) => Ok(user.clone()),
                    None => {
                        return Err(ServiceError::new(UsersError::NotFound));
                    }
                },
                Err(err) => Err(ServiceError::new(UsersError::Error(err.to_string()))),
            },
            Err(err) => Err(ServiceError::new(UsersError::Error(err.to_string()))),
        }
    }
}
