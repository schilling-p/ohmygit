use async_trait::async_trait;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, OptionalExtension};
use diesel::expression_methods::ExpressionMethods;
use uuid::Uuid;
use domain::authorization::store::AuthorizationStore;
use error::AppError;
use crate::diesel::connection::DbPool;

pub struct DieselAuthorizationStore {
    pool: DbPool,
}

impl DieselAuthorizationStore {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthorizationStore for DieselAuthorizationStore {
    async fn get_user_role_for_repository(&self, id_user: Uuid, repo_id: Uuid) -> Result<Option<String>, AppError> {
        use domain::schema::user_repository_roles::dsl::*;
        let conn = self.pool.get().await.map_err(AppError::from)?;
        let user_role: Option<String> = conn
            .interact(move |conn| user_repository_roles
                .filter(repository_id.eq(repo_id))
                .filter(user_id.eq(id_user))
                .select(role)
                .first::<String>(conn)
                .optional())
            .await
            .map_err(|e| AppError::UnexpectedError(e.to_string()))??;
        Ok(user_role)
    }
}
