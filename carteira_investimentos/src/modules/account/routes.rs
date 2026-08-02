use axum::Router;
use axum::routing::get;

use crate::app::AppState;
use crate::modules::account::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::index))
        .route("/login",
            get(handlers::login_page)
            .post(handlers::login)
        )
        .route("/logout", get(handlers::logout))
        .route("/new-account",
            get(handlers::new_account_page)
            .post(handlers::register_new_account)
        )
}
