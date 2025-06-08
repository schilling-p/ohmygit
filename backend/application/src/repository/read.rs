use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, BelongingToDsl};
use diesel::expression_methods::ExpressionMethods;
use tracing::debug;
use chrono::DateTime;

use domain::models::{Repository, User};
use domain::request::auth::{UserIdentifier};
use domain::response::repository::{RepositoryOverview, RepositoryFileInformation, CommitInformation};
use error::AppError;
use infrastructure::diesel::DbPool;
use infrastructure::git2::GitRepository;

use crate::user::read::retrieve_user_from_db;

pub async fn find_repository_by_name(pool: &DbPool, repo_name: &str) -> Result<Repository, AppError> {
    use domain::schema::repositories::dsl::*;
    let conn = pool.get().await.map_err(AppError::from)?;
    let repo_name_owned = repo_name.to_owned();
    let repo = conn
        .interact(move |conn| repositories.filter(name.eq(repo_name_owned)).select(Repository::as_select()).first::<Repository>(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;
    Ok(repo)
}

pub fn get_repo_overview(repo_path: &str, branch_name: Option<&str>) -> Result<RepositoryOverview, AppError> {
    let git_repo = GitRepository::open(repo_path)?;
    let repo_name = git_repo.get_repository_name()?;
    let head_commit = if let Some(branch_name) = branch_name {
        git_repo.get_commit_from_branch(branch_name)?
    } else {
        git_repo.get_head_commit()?
    };

    let tree = head_commit.tree()?;
    let head_commit_oid = head_commit.id();

    let mut files: Vec<RepositoryFileInformation> = Vec::new();
    for entry in tree.iter() {
        let file_name = entry.name().unwrap_or("").to_string();
        let (message, timestamp) = git_repo.get_last_commit_from_path(&file_name, head_commit_oid)?;
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

    let head_branch_name = match branch_name {
        Some(name) => name.to_owned(),
        None => git_repo.get_branch_name_from_head()?,
    };
    
    Ok(RepositoryOverview {
        head_branch_name,
        repository_name: repo_name,
        latest_commit: commit_information,
        files,
    })
}

pub async fn list_user_repositories(pool: &DbPool, user_email: &str) -> Result<Vec<Repository>, AppError> {
    debug!("listing user repositories for: {:?}", user_email);
    let user: User = retrieve_user_from_db(&pool, UserIdentifier::Email((&user_email).parse::<String>().unwrap())).await?;
    let conn = pool.get().await.map_err(AppError::from)?;
    let repos = conn
        .interact(move |conn| Repository::belonging_to(&user).select(Repository::as_select()).load(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(repos)
}