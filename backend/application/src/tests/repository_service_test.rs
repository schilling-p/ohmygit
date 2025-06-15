#![cfg(test)]
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use domain::user::model::User;
use domain::user::store::UserStore as IUserStore;
use domain::repository::store::RepositoryStore as IRepositoryStore;
use domain::request::repository::CreateRepoRequest;
use domain::request::auth::UserIdentifier;

use error::AppError;
use crate::repository::service::RepositoryService;
use crate::authorization::service::AuthorizationService;
use super::mock::{MockUserStore, MockAuthorizationStore, MockRepositoryStore, MockGitRepositoryStore};


#[tokio::test]
async fn test_create_new_user_repository_success() {
    let user_id = Uuid::new_v4();
    let user = User {
        id: user_id,
        username: "testUserName".to_string(),
        email: "testMail@test.com".to_string(),
        hashed_pw: "hashed_password".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let mut mock_user_store = MockUserStore::new();
    mock_user_store
        .expect_retrieve_user_by_identifier()
        .with(mockall::predicate::eq(UserIdentifier::Username("testUserName".to_string())))
        .times(1)
        .returning(move |_| Ok(user.clone()));

    let mut mock_repository_store = MockRepositoryStore::new();
    mock_repository_store
        .expect_retrieve_by_owner_and_name()
        .return_once(|_, _| Err(AppError::NotFound("not found".to_string())));

    mock_repository_store
        .expect_write_repo_to_db()
        .return_once(|_| Ok(()));

    let user_store: Arc<dyn IUserStore> = Arc::new(mock_user_store);
    let repository_store: Arc<dyn IRepositoryStore> = Arc::new(mock_repository_store);

    let mock_git_repository_store = Arc::new(MockGitRepositoryStore::new());
    let mock_authorization_service = Arc::new(AuthorizationService {
        user_store: user_store.clone(),
        repo_store: repository_store.clone(),
        auth_store: Arc::new(MockAuthorizationStore::new()),
    });

    let repository_service = RepositoryService {
        repo_store: repository_store,
        user_store: user_store,
        git_store: mock_git_repository_store,
        auth_service: mock_authorization_service
    };

    let test_request = CreateRepoRequest {
        repository_name: "testRepoName".to_string(),
        is_public: true,
        description: Some("testDescription".to_string()),
    };

    let result = repository_service.create_new_user_repository("testUserName".to_string(), test_request).await;

    assert!(result.is_ok());
}