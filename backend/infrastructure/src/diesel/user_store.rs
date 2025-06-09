use async_trait::async_trait;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, OptionalExtension};
use diesel::expression_methods::ExpressionMethods;

use domain::request::auth::UserIdentifier;
use domain::schema::users;
use error::AppError;
pub(crate) use domain::user::{NewUser, User, UserStore};
use crate::diesel::connection::DbPool;

pub struct DieselUserStore {
    pool: DbPool,
}

impl DieselUserStore {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserStore for DieselUserStore {
    async fn list_users(&self) -> Result<Vec<User>, AppError> {
        use domain::schema::users::dsl::*;
        let conn = self.pool.get().await?;
        let res = conn
            .interact(|conn| users.select(User::as_select()).load::<User>(conn))
            .await
            .map_err(|e| AppError::UnexpectedError(e.to_string()))?
            .map_err(AppError::from)?;
        Ok(res)
    }

    async fn retrieve_user_by_identifier(&self, user_identifier: UserIdentifier) -> Result<User, AppError> {
        use domain::schema::users::dsl::*;
        let conn = self.pool.get().await.map_err(AppError::from)?;
        let id_string = user_identifier.clone();

        let user: User = conn
            .interact(move |conn| {
                match user_identifier {
                    UserIdentifier::Email(_) => users.filter(email.eq(&id_string.into())).select(User::as_select()).first::<User>(conn),
                    UserIdentifier::Username(_) => users.filter(username.eq(&id_string.into())).select(User::as_select()).first::<User>(conn),
                    UserIdentifier::Id(_) => users.filter(id.eq(&id_string.into())).select(User::as_select()).first::<User>(conn),
                }
            })
            .await
            .map_err(|e| AppError::UnexpectedError(e.to_string()))?
            .map_err(AppError::from)?;

        Ok(user)
    }

    async fn write_user_to_db(&self, new_user: NewUser) -> Result<User, AppError> {
        let conn = self.pool.get().await.map_err(AppError::from)?;
        let user = conn
            .interact(|conn| {
                diesel::insert_into(users::table)
                    .values(new_user)
                    .returning(User::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|e| AppError::UnexpectedError(e.to_string()))?
            .map_err(AppError::from)?;
        Ok(user)
    }
}