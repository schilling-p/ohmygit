use async_trait::async_trait;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, OptionalExtension, BoolExpressionMethods};
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
        let user_vec = conn
            .interact(|conn| users.select(User::as_select()).load::<User>(conn))
            .await
            .map_err(|e| AppError::UnexpectedError(e.to_string()))?
            .map_err(AppError::from)?;

        Ok(user_vec)
    }

    async fn retrieve_user_by_identifier(&self, user_identifier: UserIdentifier) -> Result<User, AppError> {
        use domain::schema::users::dsl::*;
        let conn = self.pool.get().await.map_err(AppError::from)?;

        let user: User = conn
            .interact(move |conn| {
                match &user_identifier {
                    UserIdentifier::Email(email_str) => {
                        users.filter(email.eq(email_str)).first::<User>(conn)
                    }
                    UserIdentifier::Username(username_str) => {
                        users.filter(username.eq(username_str)).first::<User>(conn)
                    }
                    UserIdentifier::Id(uuid) => {
                        users.filter(id.eq(uuid)).first::<User>(conn)
                    }
                }
            })
            .await
            .map_err(|e| AppError::UnexpectedError(e.to_string()))?
            .map_err(AppError::from)?;

        Ok(user)
    }

    async fn retrieve_user_by_email_and_username(&self, user_email: &str, user_name: &str) -> Result<User, AppError> {
        use domain::schema::users::dsl::*;
        let conn = self.pool.get().await.map_err(AppError::from)?;
        let user_email = user_email.to_owned();
        let user_name = user_name.to_owned();
        let user: User = conn
            .interact(move |conn| users.filter(email.eq(user_email).and(username.eq(user_name))).select(User::as_select()).first::<User>(conn))
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