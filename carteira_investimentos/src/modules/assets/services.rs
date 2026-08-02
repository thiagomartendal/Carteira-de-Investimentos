use tokio::try_join;

use crate::error::AppError;
use crate::modules::assets::repository::Repository;
use crate::modules::assets::models::{Asset, OwnedAsset};

pub async fn register_asset(
    repository: &Repository,
    name: String,
    unit_value: f64,
    user_id: i64
) -> Result<(), AppError> {
    repository.create_asset(name, unit_value, user_id).await?;

    Ok(())
}

pub async fn update_asset(
    repository: &Repository,
    asset_id: i64,
    new_name: Option<String>,
    new_unit_value: Option<f64>,
    user_id: i64
) -> Result<(), AppError> {
    repository.update_asset(asset_id, new_name, new_unit_value, user_id).await?;

    Ok(())
}

pub async fn delete_asset(repository: &Repository, asset_id: i64, user_id: i64) -> Result<(), AppError> {
    repository.delete_asset(asset_id, user_id).await?;

    Ok(())
}

pub async fn purchase_asset(
    repository: &Repository,
    asset_id: i64,
    quantity: f64,
    unit_value: f64,
    user_id: i64
) -> Result<(), AppError> {
    repository.insert_owned_asset(
        user_id,
        asset_id,
        quantity,
        unit_value
    ).await?;

    Ok(())
}

pub async fn assets_page(repository: &Repository, user_id: i64) -> Result<(Vec<Asset>, Vec<OwnedAsset>, f64), AppError> {
    let (owned_assets, available_assets) = try_join!(
        repository.list_owned_assets(user_id),
        repository.list_assets()
    )?;

    let mut wallet_value: f64 = 0.0;

    for asset in &owned_assets {
        wallet_value += asset.value_delta;
    }

    Ok((available_assets, owned_assets, wallet_value))
}

pub async fn manage_page(repository: &Repository, user_id: i64) -> Result<Vec<Asset>, AppError> {
    Ok(repository.list_registrant_assets(user_id).await?)
}
