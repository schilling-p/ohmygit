use askama::Template;
use domain::models::{Repository, Organization};
use domain::response::repository::RepositoryOverview;

pub mod filters {
    pub fn length<T>(value: &Vec<T>) -> Result<usize, askama::Error> {
        Ok(value.len())
    }
}

pub use filters::length;

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
    pub username: String,
    pub overview: RepositoryOverview,
}