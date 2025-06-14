use std::sync::LazyLock;
use regex::Regex;

static REPO_NAME_REGEX: LazyLock<Regex> = LazyLock::new( || {
    Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_-]{0,31}$").unwrap()
});

static USERNAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_-]{0,15}$").unwrap()
});

pub fn is_valid_repo_name(repo_name: &str) -> bool {
    REPO_NAME_REGEX.is_match(repo_name)
}

pub fn is_valid_username(username: &str) -> bool {
    USERNAME_REGEX.is_match(username)
}