use chrono::{DateTime, Utc};
use diesel::{Associations, Identifiable, QueryId, Queryable, QueryableByName, Selectable};
use serde::Serialize;
use uuid::Uuid;
use crate::repository::model::Repository;
use crate::user::model::User;
use crate::schema::user_repository_roles;

#[derive(Selectable, Queryable, Identifiable, QueryableByName, Associations, Serialize, QueryId, Clone, Debug, PartialEq)]
#[diesel(table_name = user_repository_roles)]
#[diesel(belongs_to(Repository, foreign_key = repository_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(primary_key(user_id, repository_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRepositoryRoles {
    pub user_id: Uuid,
    pub repository_id: Uuid,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}