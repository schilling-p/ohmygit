use std::io::Write;
use tokio::process::Command;
use std::path::PathBuf;
use std::process::Stdio;
use axum::extract::{Path, Query, State};
use axum::http::{header, StatusCode};
use axum_macros::debug_handler;
use axum::response::{Response};
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, BelongingToDsl};
use diesel::expression_methods::ExpressionMethods;
use tracing::debug;
use chrono::DateTime;

use domain::models::{Repository, User};
use domain::request::repository::InfoRefsQuery;
use domain::response::repository::{RepositoryOverview, RepositoryFileInformation, CommitInformation};
use error::AppError;
use infrastructure::diesel::DbPool;
use crate::user::read::{find_user_by_email};
use infrastructure::git2::GitRepository;

pub async fn list_user_repositories(pool: &DbPool, user_email: &str)
    -> Result<Vec<Repository>, AppError> {
    debug!("listing user repositories for: {:?}", user_email);
    let user: User = find_user_by_email(&pool, &user_email).await?.0;
    let conn = pool.get().await.map_err(AppError::from)?;
    let repos = conn
        .interact(move |conn| Repository::belonging_to(&user).select(Repository::as_select()).load(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(repos)
}

#[debug_handler]
pub async fn find_user_repository(pool: State<DbPool>, Path((username, repo_name)): Path<(String, String)>, Query(query): Query<InfoRefsQuery>) -> Result<Response, AppError> {
    // figure out if the repo is public
    // if public, clone directly
    // if private, go through the authorization process
    // if the user is authorized to clone, good
    // if not, return 401

    if query.service == "git-upload-pack" {
        // TODO: find better way than unwrap_or, it does not work for either Option variant
        let repo_name_no_git = repo_name.strip_suffix(".git").unwrap_or(&repo_name);
        let repo_path = PathBuf::from(format!("/repos/{}/{}.git", username, repo_name_no_git));

        let output = run_git_advertise_refs(repo_path).await?;
        let formatted_output = format_git_advertisement(&query.service, &output);
        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/x-git-upload-pack-advertisement")
            .header(header::CACHE_CONTROL, "no-cache")
            .body(formatted_output.into())
            .unwrap();

        Ok(response)

    } else {
        Err(AppError::BadRequest(format!("unsupported service: {}", query.service)))
    }
}

#[debug_handler]
pub async fn send_user_repository(Path((username, repo_name)): Path<(String, String)>, body: axum::body::Bytes) -> Result<Response, AppError> {
    let repo_name_no_git = repo_name.strip_suffix(".git").unwrap_or(&repo_name);
    let repo_path = PathBuf::from(format!("/repos/{}/{}", username, repo_name_no_git));
    debug!("sending user repository: {:?}", &repo_path);

    let output = run_git_upload_pack(repo_path, body).await?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/x-git-upload-pack-result")
        .header(header::CACHE_CONTROL, "no-cache")
        .body(output.into())
        .unwrap();
    Ok(response)
}

async fn find_repository_by_name(pool: &DbPool, repo_name: &str) -> Result<Repository, AppError> {
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

async fn run_git_advertise_refs(repo_path: PathBuf) -> Result<Vec<u8>, AppError> {
    let output = Command::new("git")
        .arg("upload-pack")
        .arg("--stateless-rpc")
        .arg("--advertise-refs")
        .arg(repo_path)
        .output()
        .await
        .map_err(|err| AppError::InternalServerError(format!("Git spawn error: {:?}", err)))?;

    if !output.status.success() {
        return Err(AppError::InternalServerError(format!("Git error: {:?}", output.status)));
    }

    Ok(output.stdout)
}

fn format_git_advertisement(service: &str, body: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    
    let service_line = format!("# service={}\n", service);
    let pkt_line_len = service_line.len() + 4;
    write!(&mut out, "{:04x}", pkt_line_len).unwrap();
    out.extend_from_slice(service_line.as_bytes());
    out.extend_from_slice(b"0000");
    out.extend_from_slice(body);
    
    out
}

async fn run_git_upload_pack(repo_path: PathBuf, body: axum::body::Bytes) -> Result<Vec<u8>, AppError> {
    let mut child = Command::new("git")
        .arg("upload-pack")
        .arg("--stateless-rpc")
        .arg(repo_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| AppError::InternalServerError(format!("Git spawn error: {:?}", err)))?;

    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        stdin.write_all(&body).await.map_err(|err| AppError::InternalServerError(format!("Git stdin error: {:?}", err)))?;
    }

    let mut stdout = child
        .stdout
        .take()
        .ok_or(AppError::InternalServerError("Failed to capture stdout".into()))?;

    use tokio::io::AsyncReadExt;
    let mut output = Vec::new();
    stdout.read_to_end(&mut output).await.map_err(|err| AppError::InternalServerError(format!("Git stdout error: {:?}", err)))?;

    let status = child.wait().await.map_err(|err| AppError::InternalServerError(format!("Git stdout error: {:?}", err)))?;
    if !status.success() {
        return Err(AppError::InternalServerError(format!("Git error: {:?}", status)));
    }

    Ok(output)
}