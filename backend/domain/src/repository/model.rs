use chrono::{DateTime, Utc};
use diesel::{Associations, Identifiable, Insertable, QueryId, Queryable, QueryableByName, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::{repositories, branches};
use crate::user::model::User;
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
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = repositories)]
pub struct NewUserRepository {
    pub owner_id: Uuid,
    pub name: String,
    pub is_public: bool,
    pub description: Option<String>,
}

#[derive(Selectable, Queryable, QueryableByName, Identifiable, Serialize, QueryId, Associations, Clone, Debug, PartialEq)]
#[diesel(table_name = branches)]
#[diesel(belongs_to(Repository, foreign_key = repository_id))]
#[diesel(belongs_to(User, foreign_key = creator_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RepositoryBranch {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub repository_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = branches)]
pub struct NewRepositoryBranch {
    pub creator_id: Uuid,
    pub repository_id: Uuid,
    pub name: String,
}