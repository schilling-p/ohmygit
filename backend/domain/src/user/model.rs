use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, QueryId, Queryable, QueryableByName, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::users;

#[derive(Selectable, Queryable, QueryableByName, Identifiable, Serialize, QueryId, Clone, Debug, PartialEq)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub hashed_pw: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub hashed_pw: String,
}