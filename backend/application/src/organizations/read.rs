use axum::{extract::State, Json};
use axum_macros::debug_handler;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, BelongingToDsl};
use error::AppError;
use crate::user::read::find_user_by_email;
use tracing::debug;
use domain::ApiResponse;
use domain::models::{Organization, OrganizationMember};
use domain::response::organization::ListOrganizationsResponse;
use domain::request::organization::FetchOrganizationsRequest;
use domain::schema::organizations::dsl::organizations;

#[debug_handler]
pub async fn list_user_organizations(State(pool): State<deadpool_diesel::postgres::Pool>, Json(fetch_orga_request): Json<FetchOrganizationsRequest>)
    -> Result<ApiResponse, AppError> {
    debug!("listing user organizations for: {:?}", &fetch_orga_request.user_email);
    let user = find_user_by_email(&pool, &fetch_orga_request.user_email).await?.0;
    let conn = pool.get().await.map_err(AppError::from)?;
    let orgas = conn
        .interact(move |conn| OrganizationMember::belonging_to(&user).inner_join(organizations).select(Organization::as_select()).load(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(ApiResponse::Organizations(ListOrganizationsResponse {
        organizations: orgas,
        user_email: fetch_orga_request.user_email,
    }))
}