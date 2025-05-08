use askama::Template;
use domain::models::{Repository, Organization};

pub mod filters {
    pub fn length<T>(value: &Vec<T>) -> Result<usize, askama::Error> {
        Ok(value.len())
    }
}

pub use filters::length;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub name: String,
    pub repositories: Vec<Repository>,
    pub organizations: Vec<Organization>,
}