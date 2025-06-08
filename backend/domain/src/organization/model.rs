use chrono::{DateTime, Utc};
use diesel::{Identifiable, QueryId, Queryable, QueryableByName, Selectable};
use serde::Serialize;
use uuid::Uuid;
use crate::schema::organizations;

#[derive(Selectable, Queryable, QueryableByName, Identifiable, Serialize, QueryId, Clone, Debug, PartialEq)]
#[diesel(table_name = organizations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}