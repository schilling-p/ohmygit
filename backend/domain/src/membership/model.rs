use chrono::{DateTime, Utc};
use diesel::{Associations, Identifiable, QueryId, Queryable, QueryableByName, Selectable};
use serde::Serialize;
use uuid::Uuid;
use crate::schema::organizations_members;
use crate::user::model::User;
use crate::organization::model::Organization;

#[derive(Selectable, Queryable, Identifiable, QueryableByName, Associations, Serialize, QueryId, Clone, Debug, PartialEq)]
#[diesel(table_name = organizations_members)]
#[diesel(belongs_to(Organization, foreign_key = organization_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(primary_key(user_id, organization_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrganizationMember {
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}