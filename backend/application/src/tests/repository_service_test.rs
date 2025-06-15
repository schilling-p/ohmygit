#![cfg(test)]
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use git2;

use domain::user::model::User;
use domain::user::store::UserStore as IUserStore;
use domain::repository::model::Repository;
use domain::repository::store::RepositoryStore as IRepositoryStore;
use domain::filesystem::FileSystem as IFileSystem;
use domain::request::repository::CreateRepoRequest;

use error::AppError;
use crate::repository::service::RepositoryService;
use crate::authorization::service::AuthorizationService;
use super::mock::{MockUserStore, MockAuthorizationStore, MockRepositoryStore, MockGitRepositoryStore, MockFileSystem};

fn create_test_user() -> User {
    User {
        id: Uuid::new_v4(),
        username: "testUserName".to_string(),
        email: "testMail@test.com".to_string(),
        hashed_pw: "hashed_password".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_test_request() -> CreateRepoRequest {
    CreateRepoRequest {
        repository_name: "testRepoName".to_string(),
        is_public: true,
        description: Some("testDescription".to_string()),
    }
}

fn create_test_request_with_name(name: &str) -> CreateRepoRequest {
    CreateRepoRequest {
        repository_name: name.to_string(),
        is_public: true,
        description: Some("testDescription".to_string()),
    }
}

fn create_existing_repository(user_id: Uuid) -> Repository {
    Repository {
        id: Uuid::new_v4(),
        owner_id: Some(user_id),
        owner_org_id: None,
        name: "testRepoName".to_string(),
        is_public: true,
        description: Some("testRepoDescription".to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

struct TestServiceBuilder {
    mock_user_store: MockUserStore,
    mock_repository_store: MockRepositoryStore,
    mock_git_repository_store: MockGitRepositoryStore,
    mock_authorization_store: MockAuthorizationStore,
    mock_file_system: MockFileSystem,
}

impl TestServiceBuilder {
    fn new() -> Self {
        Self {
            mock_user_store: MockUserStore::new(),
            mock_repository_store: MockRepositoryStore::new(),
            mock_git_repository_store: MockGitRepositoryStore::new(),
            mock_authorization_store: MockAuthorizationStore::new(),
            mock_file_system: MockFileSystem::new(),
        }
    }

    fn with_user_found(mut self, user: User) -> Self {
        self.mock_user_store
            .expect_retrieve_user_by_identifier()
            .times(1)
            .returning(move |_| Ok(user.clone()));
        self
    }

    fn with_user_not_found(mut self) -> Self {
        self.mock_user_store
            .expect_retrieve_user_by_identifier()
            .times(1)
            .returning(|_| Err(AppError::NotFound("User not found".to_string())));
        self
    }

    fn with_user_retrieval_database_error(mut self) -> Self {
        self.mock_user_store
            .expect_retrieve_user_by_identifier()
            .times(1)
            .returning(|_| Err(AppError::DatabaseError(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new("Connection lost".to_string())
            ))));
        self
    }

    fn with_repository_exists(mut self, repo: Repository) -> Self {
        self.mock_repository_store
            .expect_retrieve_by_owner_and_name()
            .times(1)
            .returning(move |_, _| Ok(repo.clone()));
        self
    }

    fn with_repository_not_exists(mut self) -> Self {
        self.mock_repository_store
            .expect_retrieve_by_owner_and_name()
            .times(1)
            .returning(|_, _| Err(AppError::NotFound("Repository not found".to_string())));
        self
    }

    fn with_repository_check_database_error(mut self) -> Self {
        self.mock_repository_store
            .expect_retrieve_by_owner_and_name()
            .times(1)
            .returning(|_, _| Err(AppError::DatabaseError(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new("Database timeout".to_string())
            ))));
        self
    }

    fn with_repository_creation_success(mut self) -> Self {
        self.mock_repository_store
            .expect_write_repo_to_db()
            .times(1)
            .returning(|_| Ok(()));
        self
    }

    fn with_repository_creation_database_error(mut self) -> Self {
        self.mock_repository_store
            .expect_write_repo_to_db()
            .times(1)
            .returning(|_| Err(AppError::DatabaseError(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                Box::new("Database connection failed".to_string())
            ))));
        self
    }


    fn with_repository_creation_constraint_error(mut self) -> Self {
        self.mock_repository_store
            .expect_write_repo_to_db()
            .times(1)
            .returning(|_| Err(AppError::DatabaseError(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::ForeignKeyViolation,
                Box::new("Foreign key constraint failed".to_string())
            ))));
        self
    }

    fn with_git_init_success(mut self) -> Self {
        self.mock_git_repository_store
            .expect_init_bare()
            .times(1)
            .returning(|_| Box::pin(async { Ok(()) }));
        self
    }

    fn with_git_init_failure(mut self) -> Self {
        self.mock_git_repository_store
            .expect_init_bare()
            .returning(|_| Box::pin(async { Err(AppError::GitError(git2::Error::from_str("git init failed"))) }));
        self
    }

    fn with_git_init_permission_error(mut self) -> Self {
        self.mock_git_repository_store
            .expect_init_bare()
            .times(1)
            .returning(|_| Box::pin(async { Err(AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Permission denied to create git repository"
            ))) }));
        self
    }

    fn with_directory_exists(mut self) -> Self {
        self.mock_file_system
            .expect_try_exists()
            .times(1)
            .returning(|_| Ok(true));
        self
    }

    fn with_directory_not_exists(mut self) -> Self {
        self.mock_file_system
            .expect_try_exists()
            .times(1)
            .returning(|_| Ok(false));

        self.mock_file_system
            .expect_create_dir_all()
            .times(1)
            .returning(|_| Ok(()));
        self
    }

    fn with_directory_creation_failure(mut self) -> Self {
        self.mock_file_system
            .expect_try_exists()
            .times(1)
            .returning(|_| Ok(false));

        self.mock_file_system
            .expect_create_dir_all()
            .times(1)
            .returning(|_| Err(AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Permission denied"
            ))));
        self
    }

    fn with_directory_check_failure(mut self) -> Self {
        self.mock_file_system
            .expect_try_exists()
            .times(1)
            .returning(|_| Err(AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Cannot check directory"
            ))));
        self
    }

    fn build(self) -> RepositoryService {
        let user_store: Arc<dyn IUserStore> = Arc::new(self.mock_user_store);
        let repository_store: Arc<dyn IRepositoryStore> = Arc::new(self.mock_repository_store);
        let git_store = Arc::new(self.mock_git_repository_store);
        let file_system: Arc<dyn IFileSystem> = Arc::new(self.mock_file_system);

        let auth_service = Arc::new(AuthorizationService {
            user_store: user_store.clone(),
            repo_store: repository_store.clone(),
            auth_store: Arc::new(MockAuthorizationStore::new()),
        });

        RepositoryService {
            repo_store: repository_store,
            user_store,
            git_store,
            file_system,
            auth_service,

        }
    }
}

#[tokio::test]
async fn test_create_new_user_repository_success() {
    let user = create_test_user();
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_not_exists()
        .with_repository_creation_success()
        .with_directory_not_exists()
        .with_git_init_success()
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_ok(), "Expected repository creation to succeed");
}

#[tokio::test]
async fn test_create_new_user_repository_private_repository_success() {
    let user = create_test_user();
    let mut request = create_test_request();
    request.is_public = false;
    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_not_exists()
        .with_repository_creation_success()
        .with_directory_not_exists()
        .with_git_init_success()
        .build();

    let result = service.create_new_user_repository("testUsername".to_string(), request).await;
    assert!(result.is_ok(), "Expected repository creation to succeed");
}

#[tokio::test]
async fn test_create_new_user_repository_private_no_description_success() {
    let user = create_test_user();
    let mut request = create_test_request_with_name("testRepoName");
    request.description = None;
    request.is_public = false;
    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_not_exists()
        .with_repository_creation_success()
        .with_directory_not_exists()
        .with_git_init_success()
        .build();

    let result = service.create_new_user_repository("testUsername".to_string(), request).await;
    assert!(result.is_ok(), "Expected repository creation to succeed");
}

#[tokio::test]
async fn test_create_new_user_repository_with_no_description() {
    let mut request = create_test_request_with_name("testRepoName");
    request.description = None;
    let user = create_test_user();
    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_not_exists()
        .with_repository_creation_success()
        .with_directory_not_exists()
        .with_git_init_success()
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_ok(), "Expected repository creation to succeed");
}

#[tokio::test]
async fn test_create_new_user_repository_user_not_found() {
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_not_found()
        .build();

    let result = service.create_new_user_repository("nonexistentUser".to_string(), request).await;

    assert!(result.is_err(), "Expected error when user not found");
    match result.unwrap_err() {
        AppError::NotFound(msg) => assert!(msg.contains("User not found")),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_create_new_user_repository_repo_already_exists() {
    let user = create_test_user();
    let existing_repo = create_existing_repository(user.id);
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_exists(existing_repo)
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_err(), "Expected error when repository already exists");
    match result.unwrap_err() {
        AppError::RepositoryAlreadyExists => {},
        _ => panic!("Expected RepositoryAlreadyExists error"),
    }
}

#[tokio::test]
async fn test_create_repository_directory_already_exists() {
    let user = create_test_user();
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_not_exists()
        .with_repository_creation_success()
        .with_directory_exists()
        .with_git_init_success()
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_ok(), "Expected repository creation to succeed when directory exists");
}

#[tokio::test]
async fn test_create_repository_directory_creation_failure() {
    let user = create_test_user();
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_not_exists()
        .with_repository_creation_success()
        .with_directory_creation_failure()
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_err(), "Expected error when directory creation fails");
    match result.unwrap_err() {
        AppError::IoError(_) => {},
        _ => panic!("Expected IoError"),
    }
}

#[tokio::test]
async fn test_create_repository_directory_check_failure() {
    let user = create_test_user();
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_not_exists()
        .with_repository_creation_success()
        .with_directory_check_failure()
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_err(), "Expected error when directory check fails");
    match result.unwrap_err() {
        AppError::IoError(_) => {},
        _ => panic!("Expected IoError"),
    }
}

#[tokio::test]
async fn test_create_repository_user_retrieval_database_error() {
    let request = create_test_request();
    let service = TestServiceBuilder::new()
        .with_user_retrieval_database_error()
        .build();

    let result = service.create_new_user_repository("testUser".to_string(), request).await;

    assert!(result.is_err(), "Expected error when user retrieval fails");
    match result.unwrap_err() {
        AppError::DatabaseError(_) => {},
        _ => panic!("Expected DatabaseError"),
    }
}

#[tokio::test]
async fn test_create_new_user_repository_database_error() {
    let user = create_test_user();
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_not_exists()
        .with_repository_creation_database_error()
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_err(), "Expected error when database fails");
    match result.unwrap_err() {
        AppError::DatabaseError(err) => {},
        _ => panic!("Expected UnexpectedError"),
    }
}

#[tokio::test]
async fn test_create_new_user_repository_already_exists() {
    let user = create_test_user();
    let existing_repo = create_existing_repository(user.id);
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_exists(existing_repo)
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_err(), "Expected error when repository already exists");
    match result.unwrap_err() {
        AppError::RepositoryAlreadyExists => {},
        _ => panic!("Expected RepositoryAlreadyExists error"),
    }
}

#[tokio::test]
async fn test_create_new_user_repository_check_database_error() {
    let user = create_test_user();
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_check_database_error()
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_err(), "Expected error when repository existence check fails");
    match result.unwrap_err() {
        AppError::DatabaseError(_) => {},
        _ => panic!("Expected DatabaseError"),
    }
}

#[tokio::test]
async fn test_create_new_user_repository_database_write_error() {
    let user = create_test_user();
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_not_exists()
        .with_repository_creation_database_error()
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_err(), "Expected error when database write fails");
    match result.unwrap_err() {
        AppError::DatabaseError(_) => {},
        _ => panic!("Expected DatabaseError"),
    }
}

#[tokio::test]
async fn test_create_repository_constraint_violation() {
    let user = create_test_user();
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_not_exists()
        .with_repository_creation_constraint_error()
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_err(), "Expected error when constraint violation occurs");
    match result.unwrap_err() {
        AppError::DatabaseError(_) => {},
        _ => panic!("Expected DatabaseError"),
    }
}

#[tokio::test]
async fn test_create_repository_git_init_failure() {
    let user = create_test_user();
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_not_exists()
        .with_repository_creation_success()
        .with_directory_not_exists()
        .with_git_init_failure()
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_err(), "Expected error when git initialization fails");
    match result.unwrap_err() {
        AppError::GitError(_) => {},
        _ => panic!("Expected GitError"),
    }
}

#[tokio::test]
async fn test_create_repository_git_permission_error() {
    let user = create_test_user();
    let request = create_test_request();

    let service = TestServiceBuilder::new()
        .with_user_found(user)
        .with_repository_not_exists()
        .with_repository_creation_success()
        .with_directory_not_exists()
        .with_git_init_permission_error()
        .build();

    let result = service.create_new_user_repository("testUserName".to_string(), request).await;

    assert!(result.is_err(), "Expected error when git init has permission issues");
    match result.unwrap_err() {
        AppError::IoError(_) => {},
        _ => panic!("Expected IoError"),
    }
}