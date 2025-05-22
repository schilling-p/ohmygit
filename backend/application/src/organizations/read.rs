use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, BelongingToDsl};
use error::AppError;
use crate::user::read::retrieve_user_from_db;
use tracing::debug;
use domain::models::{Organization, OrganizationMember};
use domain::request::auth::UserIdentifier;
use domain::schema::organizations::dsl::organizations;
use infrastructure::diesel::DbPool;

pub async fn list_user_organizations(pool: &DbPool, user_email: &str) -> Result<Vec<Organization>, AppError> {
    debug!("listing user organizations for: {:?}", user_email);
    let user = retrieve_user_from_db(&pool, UserIdentifier::Email((&user_email).parse::<String>().unwrap())).await?.0;
    let conn = pool.get().await.map_err(AppError::from)?;
    let orgas = conn
        .interact(move |conn| OrganizationMember::belonging_to(&user).inner_join(organizations).select(Organization::as_select()).load(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(orgas)
}