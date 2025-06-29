use async_trait::async_trait;
use uuid::Uuid;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, OptionalExtension, BoolExpressionMethods};
use diesel::expression_methods::ExpressionMethods;
use domain::repository::model::{Repository, NewUserRepository, NewRepositoryBranch};
use domain::repository::store::RepositoryStore;

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
    
    async fn retrieve_by_owner_and_name(&self, repo_owner_id: Uuid, repo_name: &str) -> Result<Repository, AppError> {
        use domain::schema::repositories::dsl::*;
        let conn = self.pool.get().await.map_err(AppError::from)?;
        let repo_name = repo_name.to_owned();
        let repo_owner_id = repo_owner_id.to_owned();
        let repo = conn
            .interact(move |conn| repositories.filter(owner_id.eq(repo_owner_id).and(name.eq(repo_name))).select(Repository::as_select()).first::<Repository>(conn))
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
        conn.interact(move |conn| {
                diesel::insert_into(repositories)
                    .values(new_repo)
                    .execute(conn)
            })
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;
        Ok(())
    }
    
    async fn write_repo_branch_to_db(&self, new_branch: NewRepositoryBranch) -> Result<(), AppError> {
        use domain::schema::branches::dsl::*;
        let conn = self.pool.get().await.map_err(AppError::from)?;
        conn.interact(move |conn| {
            diesel::insert_into(branches)
                .values(new_branch)
                .execute(conn)
        })
            .await
            .map_err(|e| AppError::UnexpectedError(e.to_string()))?
            .map_err(AppError::from)?;
        Ok(())
    }
}