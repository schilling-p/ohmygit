use diesel::{Queryable, QueryableByName, Selectable};
use serde::{Serialize};
use chrono::{DateTime, Utc};
use crate::schema::users;

#[derive(Serialize, Selectable, Queryable, QueryableByName)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub name: String,
    pub email: String,
    pub hashed_pw: String,
    pub created_at: DateTime<Utc>,
}