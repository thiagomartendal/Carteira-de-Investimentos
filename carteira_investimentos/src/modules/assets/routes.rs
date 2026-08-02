use axum::Router;
use axum::{routing::get, routing::delete};

use crate::app::AppState;
use crate::modules::assets::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assets", 
            get(handlers::assets)
            .post(handlers::purchase_asset)
        )
        .route("/manage",
            get(handlers::manage)
            .post(handlers::register_asset)
            .patch(handlers::update_asset)
        )
        .route("/manage/{asset_id}", delete(handlers::delete_asset))
}
