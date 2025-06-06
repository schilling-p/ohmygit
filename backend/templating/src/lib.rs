use askama::Template;
use domain::models::{Repository, Organization};
use domain::response::repository::RepositoryOverview;
use serde::Serialize;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub username: String,
    pub repositories: Vec<Repository>,
    pub organizations: Vec<Organization>,
}

#[derive(Template)]
#[template(path = "repository.html")]
pub struct RepositoryTemplate {
    pub repository_name: String,
    pub username: String,
    pub overview: RepositoryOverview,
}

#[derive(Template, Serialize)]
#[template(path = "create_repository.html")]
pub struct CreateRepositoryTemplate {
    pub username: String,
    pub repositories: Vec<String>
}