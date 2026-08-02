use axum::http::StatusCode;
use axum_extra::extract::cookie::PrivateCookieJar;
use axum::response::{IntoResponse, Redirect};
use axum::extract::State;
use axum::Form;
use sqlx::PgPool;

use crate::modules::auth::user::User;
use crate::error::AppError;
use crate::modules::account::repository::Repository;
use crate::modules::account::services;
use crate::modules::utils::state_response::StateResponse;
use crate::modules::account::dtos::{LoginForm, NewAccountForm};
use crate::modules::account::views::{LoginPage, NewAccountPage};

/*########## Funções de Renderização das Páginas HTML ##########*/

pub async fn index(maybe_user: Option<User>) -> Result<Redirect, AppError> {
    match maybe_user {
        Some(user) => {
            if user.user_type == 1 {
                Ok(Redirect::to("/assets"))
            } else {
                Ok(Redirect::to("/manage"))
            }
        },
        None => Ok(Redirect::to("/login"))
    }
}

pub async fn login_page(maybe_user: Option<User>) -> Result<impl IntoResponse, AppError> {
    match maybe_user {
        Some(user) => {
            if user.user_type == 1 {
                Ok(StateResponse::Redirect { route: String::from("/assets"), status: StatusCode::SEE_OTHER })
            } else {
                Ok(StateResponse::Redirect { route: String::from("/manage"), status: StatusCode::SEE_OTHER })
            }
        },
        None => {
            let html = LoginPage::new(String::new()).html()?;
            Ok(StateResponse::RenderObject { html, status: StatusCode::UNAUTHORIZED })
        }
    }
}

pub async fn new_account_page(maybe_user: Option<User>) -> Result<impl IntoResponse, AppError> {
    match maybe_user {
        Some(user) => {
            if user.user_type == 1 {
                Ok(StateResponse::Redirect { route: String::from("/assets"), status: StatusCode::SEE_OTHER })
            } else {
                Ok(StateResponse::Redirect { route: String::from("/manage"), status: StatusCode::SEE_OTHER})
            }
        },
        None => {
            let html = NewAccountPage::new(String::new()).html()?;
            Ok(StateResponse::RenderObject { html, status: StatusCode::OK })
        }
    }
}

/*########## Funções de Backend ##########*/

pub async fn login(
    jar: PrivateCookieJar,
    State(pool): State<PgPool>,
    Form(request): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let repository = Repository::new(pool);
    let save_session: bool = request.save_session.as_deref() == Some("save");

    let auth = services::auth_user(&repository, request.email, request.password, save_session).await;
    match auth {
        Ok(cookie) => {
            Ok(StateResponse::RedirectWithCookie { jar_cookie: jar.add(cookie.into_owned()), route: String::from("/"), status: StatusCode::SEE_OTHER })
        },
        Err(error) => {
            let html = LoginPage::new(String::from(error.to_string())).html()?;

            return Ok(StateResponse::RenderObject { html: html, status: error.into_response().status() });
        }
    }
}

pub async fn logout(jar: PrivateCookieJar) -> impl IntoResponse {
    (jar.remove("token"), (StatusCode::SEE_OTHER, Redirect::to("/login")).into_response())
}

pub async fn register_new_account(
    jar: PrivateCookieJar,
    State(pool): State<PgPool>,
    Form(request): Form<NewAccountForm>
) -> Result<impl IntoResponse, AppError> {
    let repository = Repository::new(pool);

    let registered = services::register_account(&repository, request.email, request.username, request.password, request.user_type).await;
    match registered {
        Ok(cookie) => {
            Ok(StateResponse::RedirectWithCookie { jar_cookie: jar.add(cookie), route: String::from("/"), status: StatusCode::SEE_OTHER })
        },
        Err(error) => {
            let html = NewAccountPage::new(String::from(error.to_string())).html()?;

            return Ok(StateResponse::RenderObject { html: html, status: error.into_response().status() });
        }
    }
}
