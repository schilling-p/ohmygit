use std::pin::Pin;
use std::future::Future;
use uuid::Uuid;
use async_trait::async_trait;

use domain::user::model::{User, NewUser};
use domain::request::auth::UserIdentifier;
use domain::authorization::store::AuthorizationStore as IAuthorizationStore;
use domain::repository::model::{Repository, NewUserRepository, NewRepositoryBranch};
use domain::repository::store::RepositoryStore as IRepositoryStore;
use domain::repository::git_store::GitRepositoryStore as IGitRepositoryStore;
use domain::response::repository::RepositoryOverview;
use domain::user::store::UserStore as IUserStore;
use error::AppError;

mockall::mock! {
    pub UserStore {}

    #[async_trait]
    impl IUserStore for UserStore {
        async fn list_users(&self) -> Result<Vec<User>, AppError>;
        async fn retrieve_user_by_identifier(&self, user_identifier: UserIdentifier) -> Result<User, AppError>;
        async fn retrieve_user_by_email_and_username(&self, user_email: &str, user_name: &str) -> Result<User, AppError>;
        async fn write_user_to_db(&self, new_user: NewUser) -> Result<User, AppError>;
    }
}

mockall::mock! {
    pub AuthorizationStore {}

    #[async_trait]
    impl IAuthorizationStore for AuthorizationStore {
        async fn get_user_role_for_repository(&self, id_user: Uuid, repo_id: Uuid) -> Result<Option<String>, AppError>;
    }
}

mockall::mock! {
    pub RepositoryStore {}

    #[async_trait]
    impl IRepositoryStore for RepositoryStore {
        async fn retrieve_by_name(&self, repo_name: &str) -> Result<Repository, AppError>;
        async fn retrieve_by_owner_and_name(&self, owner_id: Uuid, repo_name: &str) -> Result<Repository, AppError>;
        async fn list_user_repositories(&self, user_id: Uuid) -> Result<Vec<Repository>, AppError>;
        async fn write_repo_to_db(&self, new_repo: NewUserRepository) -> Result<(), AppError>;
        async fn write_repo_branch_to_db(&self, new_branch: NewRepositoryBranch) -> Result<(), AppError>;
    }
}

mockall::mock! {
    pub GitRepositoryStore {}

    impl IGitRepositoryStore for GitRepositoryStore {
        fn init_bare<'a>(&'a self, path: &'a str) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'static>>;
        fn list_local_branches<'a>(&'a self, path: &'a str) -> Pin<Box<dyn Future<Output = Result<Vec<String>, AppError>> + Send + 'static>>;
        fn create_branch<'a>(&'a self, path: &'a str, new_branch: &'a str, base_branch: &'a str, switch_head: bool) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'static>>;
        fn get_repo_overview<'a>(&'a self, path: &'a str, branch_name: Option<&'a String>) -> Pin<Box<dyn Future<Output = Result<RepositoryOverview, AppError>> + Send + 'static>>;
    }
}