use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::{Form, extract::Path};
use sqlx::PgPool;

use crate::modules::auth::user::User;
use crate::error::AppError;
use crate::modules::assets::services;
use crate::modules::assets::repository::Repository;
use crate::modules::assets::views::{AssetsPage, ManagePage};
use crate::modules::utils::state_response::StateResponse;
use crate::modules::assets::dtos::{RegisterAssetForm, UpdateAssetJson, PurchaseAssetForm};

/*########## Funções de Renderização das Páginas HTML ##########*/

pub async fn assets(user: User, State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    if user.user_type == 2 {
        return Ok(StateResponse::Redirect { route: String::from("/"), status: StatusCode::SEE_OTHER });
    }

    let repository = Repository::new(pool);

    let (available_assets, owned_assets, wallet_value) = services::assets_page(&repository, user.id()).await?;

    let html = AssetsPage::new(
        owned_assets,
        available_assets,
        user,
        wallet_value
    ).html()?;

    Ok(StateResponse::RenderObject { html: html, status: StatusCode::OK })
}

pub async fn manage(user: User, State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    if user.user_type == 1 {
        return Ok(StateResponse::Redirect { route: String::from("/"), status: StatusCode::SEE_OTHER });
    }

    let repository = Repository::new(pool);

    let available_assets = services::manage_page(&repository, user.id()).await?; 

    let html = ManagePage::new(available_assets, user).html()?;

    Ok(StateResponse::RenderObject { html: html, status: StatusCode::OK })
}

/*########## Funções de Backend ##########*/

pub async fn register_asset(
    user: User,
    State(pool): State<PgPool>,
    Form(request): Form<RegisterAssetForm>
) ->Result<impl IntoResponse, AppError> {
    let repository = Repository::new(pool);
    
    services::register_asset(&repository, request.name, request.unit_value, user.id).await?;
    
    Ok(StateResponse::Redirect { route: String::from("/manage"), status: StatusCode::SEE_OTHER })
}

pub async fn update_asset(
    user: User,
    State(pool): State<PgPool>,
    Json(request): Json<UpdateAssetJson>
) -> Result<(), AppError> {
    let repository = Repository::new(pool);

    services::update_asset(&repository, request.asset_id, request.new_name, request.new_unit_value, user.id).await
}

pub async fn delete_asset(
    user: User,
    State(pool): State<PgPool>,
    Path(asset_id): Path<u64>
) -> Result<(), AppError> {
    let repository = Repository::new(pool);

    services::delete_asset(&repository, asset_id as i64, user.id).await
}

pub async fn purchase_asset(
    user: User,
    State(pool): State<PgPool>,
    Form(request): Form<PurchaseAssetForm>
) -> Result<impl IntoResponse, AppError> {
    let repository = Repository::new(pool);

    services::purchase_asset(&repository, request.asset_id, request.quantity, request.unit_value, user.id()).await?;

    Ok((StatusCode::SEE_OTHER, Redirect::to("/assets")).into_response())
}
