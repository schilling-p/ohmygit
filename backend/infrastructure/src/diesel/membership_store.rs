use async_trait::async_trait;
use std::sync::Arc;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, BelongingToDsl};
use diesel::expression_methods::ExpressionMethods;
use uuid::Uuid;
use domain::membership::store::MembershipStore;
use domain::membership::model::OrganizationMember;
use domain::organization::model::Organization;
use domain::request::auth::UserIdentifier;
use domain::schema::organizations::dsl::organizations;
use error::AppError;
use crate::diesel::user_store::UserStore;
use crate::diesel::connection::DbPool;


pub struct DieselMembershipStore {
    pool: DbPool,
    user_store: Arc<dyn UserStore>,
}

impl DieselMembershipStore {
    pub fn new(pool: DbPool, user_store: Arc<dyn UserStore>) -> Self {
        Self { pool , user_store}
    }
}

#[async_trait]
impl MembershipStore for DieselMembershipStore {
    async fn list_organizations_for_user(&self, user_id: Uuid) -> Result<Vec<Organization>, AppError> {
        let user = self.user_store.retrieve_user_by_identifier(UserIdentifier::Id(user_id)).await?;
        let conn = self.pool.get().await.map_err(AppError::from)?;
        let orgas = conn
            .interact(move |conn| OrganizationMember::belonging_to(&user).inner_join(organizations).select(Organization::as_select()).load(conn))
            .await
            .map_err(|e| AppError::UnexpectedError(e.to_string()))?
            .map_err(AppError::from)?;

        Ok(orgas)
    }
}