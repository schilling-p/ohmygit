use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use error::AppError;

pub async fn run_git_advertise_refs(service: &str, repo_path: PathBuf) -> Result<Vec<u8>, AppError> {
    let output = Command::new(service)
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

pub async fn run_git_pack(service: &str, repo_path: PathBuf, body: axum::body::Bytes) -> Result<Vec<u8>, AppError> {
    let mut child = Command::new(service)
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
        return Err(AppError::InternalServerError(format!("Git execution error: {:?}", status)));
    }

    Ok(output)
}