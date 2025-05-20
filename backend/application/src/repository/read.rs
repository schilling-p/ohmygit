use axum::{extract::Path};
use axum_macros::debug_handler;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, BelongingToDsl};
use tracing::debug;
use chrono::DateTime;

use domain::models::Repository;
use domain::request::repository::{RepositoryPath};
use domain::ApiResponse;
use domain::response::repository::{RepositoryOverview, RepositoryFileInformation, CommitInformation};
use error::AppError;
use infrastructure::diesel::DbPool;
use crate::user::read::find_user_by_email;
use infrastructure::git2::GitRepository;

pub async fn list_user_repositories(pool: &DbPool, user_email: &str)
    -> Result<Vec<Repository>, AppError> {
    debug!("listing user repositories for: {:?}", user_email);
    let user = find_user_by_email(&pool, &user_email).await?.0;
    let conn = pool.get().await.map_err(AppError::from)?;
    let repos = conn
        .interact(move |conn| Repository::belonging_to(&user).select(Repository::as_select()).load(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(repos)
}

pub fn get_repo_overview(repo_path: &str) -> Result<RepositoryOverview, AppError> {
    let git_repo = GitRepository::open(repo_path)?;
    let repo_name = git_repo.get_repository_name()?;
    let head_commit = git_repo.get_head_commit()?;
    let tree = head_commit.tree()?;

    let mut files: Vec<RepositoryFileInformation> = Vec::new();
    for entry in tree.iter() {
        let file_name = entry.name().unwrap_or("").to_string();
        let (message, timestamp) = git_repo.get_last_commit_for_path(&file_name)?;
        files.push(RepositoryFileInformation {
            file_name,
            last_commit_message: message,
            last_commit_time: timestamp,
        });
    }

    let head_commit_time = DateTime::from_timestamp(head_commit.time().seconds(), 0).unwrap();
    let commit_information = CommitInformation {
        commit_message: head_commit.message().unwrap_or("no commit yet").to_string(),
        commit_time: head_commit_time.to_string(),
    };
    
    Ok(RepositoryOverview {
        repository_name: repo_name,
        latest_commit: commit_information,
        files,
    })
}