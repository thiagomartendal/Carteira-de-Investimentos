use std::convert::Infallible;

use axum::extract::FromRequestParts;
use axum_extra::extract::cookie::PrivateCookieJar;
use jwt_simple::algorithms::{HS256Key, MACLike};
use jwt_simple::claims::Claims;
use jwt_simple::reexports::coarsetime::Duration;
use password_auth::VerifyError;
use serde::{Deserialize, Serialize};

use crate::app::AppState;
use crate::error::AppError;
use crate::modules::account::repository::Repository;

const SECRET_KEY: &[u8] = b"Segredo-em-um-dia-qualquer"; // Chave secreta para token JWT

pub struct UnauthenticatedUser {
    email: String,
    username: String,
    password: String,
    user_type: i32
}

impl UnauthenticatedUser {
    pub fn new(email: String, username: String, password: String, user_type: i32) -> Self {
        Self { email, username, password, user_type }
    }

    pub async fn check_user_email(&self, repository: &Repository) -> Result<bool, AppError> {
        match repository.get_user_by_email(&self.email).await? {
            Some(_) => Err(AppError::EmailTaken),
            None => Ok(false),
        }
    }

    pub async fn authenticate(&self, repository: &Repository) -> Result<User, AppError> {
        let user_record = match repository.get_user_by_email(&self.email).await? {
            Some(user_record) => user_record,
            None => return Err(AppError::UserDoesNotExist),
        };

        match password_auth::verify_password(&self.password, &user_record.password_hash) {
            Ok(()) => Ok(User::new(user_record.id, user_record.email, user_record.username, user_record.user_type)),
            Err(VerifyError::PasswordInvalid) => Err(AppError::InvalidCredentials),
            Err(VerifyError::Parse(err)) => panic!("Falha no algoritmo de hash: {err}"),
        }
    }

    pub async fn register(self, repository: &Repository) -> Result<User, AppError> {
        let password_hash = password_auth::generate_hash(self.password);
        let user_record = match repository.add_user(&self.email, &self.username, &password_hash, self.user_type).await {
            Ok(user_record) => user_record,
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                return Err(AppError::EmailTaken);
            }
            Err(err) => return Err(AppError::Database(err)),
        };

        Ok(User::new(user_record.id, user_record.email, user_record.username, user_record.user_type))
    }
}

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub user_type: i32
}

impl User {
    fn new(id: i64, email: String, username: String, user_type: i32) -> Self {
        Self { id, email, username, user_type }
    }

    pub const fn id(&self) -> i64 {
        self.id
    }

    pub const fn email(&self) -> &String {
        &self.email
    }

    pub const fn username(&self) -> &String {
        &self.username
    }

    pub const fn user_type(&self) -> &i32 {
        &self.user_type
    }

    pub fn auth_token(self, save_session: bool) -> Result<String, AppError> {
        let duration = match save_session {
            true => Duration::from_days(365),
            false => Duration::from_days(1) 
        };

        let key = HS256Key::from_bytes(SECRET_KEY);
        let claims = Claims::with_custom_claims(UserClaims::from(self), duration);
        let token = key.authenticate(claims)?;
        Ok(token)
    }

    pub fn from_auth_token(token: &str) -> Result<Self, AppError> {
        let key = HS256Key::from_bytes(SECRET_KEY);
        let claims: UserClaims = key.verify_token(token, None)?.custom;
        Ok(Self::new(claims.id, claims.email, claims.username, claims.user_type))
    }
}

impl FromRequestParts<AppState> for User {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = PrivateCookieJar::from_headers(&parts.headers, state.key.clone());
        
        /* Recuperação de cookie do CookieJar
        let token = match jar.get("token") {
            Some(token) => token.value(),
            None => return Err(AppError::MissingAuthorization)
        };
        */

        // Recuperação de cookie do PrivateCookieJar
        let token = match jar.get("token").map(|cookie| cookie.value().to_owned()) {
            Some(token) => token,
            None => return Err(AppError::MissingAuthorization)
        };

        User::from_auth_token(token.as_str())
    }
}

impl FromRequestParts<AppState> for Option<User> {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(User::from_request_parts(parts, state).await.ok())
    }
}

#[derive(Serialize, Deserialize)]
struct UserClaims {
    id: i64,
    email: String,
    username: String,
    user_type: i32
}

impl From<User> for UserClaims {
    fn from(User {id, email, username, user_type}: User) -> Self {
        Self { id, email, username, user_type }
    }
}
