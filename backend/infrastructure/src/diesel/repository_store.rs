use async_trait::async_trait;
use uuid::Uuid;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, OptionalExtension};
use diesel::expression_methods::ExpressionMethods;
use domain::repository::model::{Repository, NewUserRepository};
use domain::repository::store::RepositoryStore;
use domain::schema::repositories;


use crate::diesel::connection::DbPool;
use error::AppError;

pub struct DieselRepositoryStore {
    pool: DbPool,
}

impl DieselRepositoryStore {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RepositoryStore for DieselRepositoryStore {
    async fn retrieve_by_name(&self, repo_name: &str) -> Result<Repository, AppError> {
        use domain::schema::repositories::dsl::*;
        let conn = self.pool.get().await.map_err(AppError::from)?;
        let repo_name_owned = repo_name.to_owned();
        let repo = conn
            .interact(move |conn| repositories.filter(name.eq(repo_name_owned)).select(Repository::as_select()).first::<Repository>(conn))
            .await
            .map_err(|e| AppError::UnexpectedError(e.to_string()))?
            .map_err(AppError::from)?;
        Ok(repo)
    }

    async fn list_user_repositories(&self, user_id: Uuid) -> Result<Vec<Repository>, AppError> {
        use domain::schema::repositories::dsl::*;
        let conn = self.pool.get().await.map_err(AppError::from)?;
        let repos = conn
            .interact(move |conn| repositories.filter(owner_id.eq(user_id)).select(Repository::as_select()).load::<Repository>(conn))
            .await
            .map_err(|e| AppError::UnexpectedError(e.to_string()))?
            .map_err(AppError::from)?;
        Ok(repos)
    }

    async fn write_repo_to_db(&self, new_repo: NewUserRepository) -> Result<(), AppError> {
        use domain::schema::repositories::dsl::*;
        let conn = self.pool.get().await.map_err(AppError::from)?;
        let repo = conn
            .interact(move |conn| {
                diesel::insert_into(repositories)
                    .values(new_repo)
                    .returning(Repository::as_select())
                    .get_result(conn)
            })
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;
        Ok(())
    }
}