use axum_extra::extract::cookie::{Cookie, SameSite};
use time::Duration;

use crate::modules::auth::user::{UnauthenticatedUser, User};
use crate::error::AppError;
use crate::modules::account::repository::Repository;

pub async fn auth_user(repository: &Repository, email: String, password: String, save_session: bool) -> Result<Cookie<'static>, AppError> {
    let unauth_user = UnauthenticatedUser::new(email, String::new(), password, 0);
    let user = match unauth_user.authenticate(repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExist) => return Err(AppError::UserDoesNotExist),
        Err(other_err) => return Err(other_err)
    };

    let token = user.auth_token(save_session)?;
    let mut cookie: Cookie<'static> = Cookie::build(("token", token))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Strict)
        .secure(true)
        .build(); 

    if save_session { 
        cookie.set_max_age(Duration::weeks(52));
    }

    Ok(cookie)
}

pub async fn register_account(
    repository: &Repository,
    email: String,
    username: String,
    password: String,
    user_type: i32
) -> Result<Cookie<'static>, AppError> {
    let unauth_user = UnauthenticatedUser::new(email, username, password, user_type);
    let user: Result<User, AppError> = match unauth_user.check_user_email(&repository).await {
        Ok(_) => Ok(unauth_user.register(&repository).await?),
        Err(AppError::EmailTaken) => Err(AppError::EmailTaken),
        Err(other_err) => Err(other_err)
    };

    let token = user?.auth_token(false)?;
    let cookie = Cookie::build(("token", token))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Strict)
        .secure(true)
        .build();

    Ok(cookie)
}
