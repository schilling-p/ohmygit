use diesel::{Associations, Identifiable, Insertable, QueryId, Queryable, QueryableByName, Selectable};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::schema::{users, repositories};

#[derive(Selectable, Queryable, QueryableByName, Identifiable, Serialize, QueryId, Clone, Debug, PartialEq)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub hashed_pw: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Selectable, Queryable, QueryableByName, Identifiable, Serialize, QueryId, Associations, Clone, Debug, PartialEq)]
#[diesel(table_name = repositories)]
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Repository {
    pub id: Uuid,
    pub owner_id: Option<Uuid>,
    pub owner_org_id: Option<Uuid>,
    pub name: String,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub hashed_pw: String,
}