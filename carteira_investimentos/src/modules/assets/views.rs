use askama::Template;

use crate::error::AppError;
use crate::modules::auth::user::User;
use crate::modules::assets::models::{Asset, OwnedAsset};

#[derive(Template)]
#[template(path = "assets.html")]
pub struct AssetsPage {
    owned_assets: Vec<OwnedAsset>,
    available_assets: Vec<Asset>,
    user: User,
    wallet_value: f64
}

impl AssetsPage {
    pub fn new(
        owned_assets: Vec<OwnedAsset>,
        available_assets: Vec<Asset>,
        user: User,
        wallet_value: f64
    ) -> Self {
        Self {
            owned_assets,
            available_assets,
            user,
            wallet_value
        }        
    }

    pub fn html(self) -> Result<String, AppError> {
        Ok(self.render()?)
    }
}

#[derive(Template)]
#[template(path = "manage.html")]
pub struct ManagePage {
    available_assets: Vec<Asset>,
    user: User
}

impl ManagePage {
    pub fn new(available_assets: Vec<Asset>, user: User) -> Self {
        Self {
            available_assets,
            user
        }        
    }

    pub fn html(self) -> Result<String, AppError> {
        Ok(self.render()?)
    }
}

// Filtro de exibição para data e hora
pub mod filters {
    use askama;
    use time::{
        OffsetDateTime,
        format_description::StaticFormatDescription,
        macros::format_description,
        UtcOffset
    };

    #[askama::filter_fn]
    pub fn human_datetime(
        datetime: &OffsetDateTime,
        _env: &dyn askama::Values, 
    ) -> askama::Result<String> {
        let utc3 = UtcOffset::from_hms(-3, 0, 0).unwrap();
        
        const HUMAN_READABLE_FORMAT: StaticFormatDescription = format_description!(version = 2, "[day]/[month]/[year] às [hour]:[minute]");
        datetime.to_offset(utc3).format(HUMAN_READABLE_FORMAT).map_err(askama::Error::custom)
    }
}
