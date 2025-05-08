use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, BelongingToDsl};
use error::AppError;
use crate::user::read::find_user_by_email;
use tracing::debug;
use domain::models::{Organization, OrganizationMember};
use domain::schema::organizations::dsl::organizations;
use infrastructure::diesel::DbPool;

pub async fn list_user_organizations(pool: &DbPool, user_email: &str) -> Result<Vec<Organization>, AppError> {
    debug!("listing user organizations for: {:?}", user_email);
    let user = find_user_by_email(&pool, &user_email).await?.0;
    let conn = pool.get().await.map_err(AppError::from)?;
    let orgas = conn
        .interact(move |conn| OrganizationMember::belonging_to(&user).inner_join(organizations).select(Organization::as_select()).load(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(orgas)
}