use axum::{Json, http::StatusCode, response::{IntoResponse, Redirect}};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("O cabeçalho de autorização não foi encontrado.")]
    MissingAuthorization,

    #[error("Credenciais inválidas.")]
    InvalidCredentials,

    #[error("O ativo não foi encontrado.")]
    AssetDoesNotExist,

    #[error("O usuário não está cadastrado.")]
    UserDoesNotExist,

    #[error("O email já está em uso.")]
    EmailTaken,

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Template(#[from] askama::Error),
    
    #[error(transparent)]
    Jwt(#[from] jwt_simple::Error)
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };

        match self {
            Self::MissingAuthorization => {
                // Para acesso indevído de rotas protegidas, basta apenas redirecionar, sem a
                // necessidade de exibir a mensagem de erro
                Redirect::to("/").into_response()
            }

            Self::Jwt(error) => {
                match error.downcast_ref::<jwt_simple::JWTError>() {
                    // Caso o token expire, também pode-se apenas redirecionar diretamente para a
                    // página de logout para se excluir o cookie (caso este ainda exista), e então
                    // retornar a página de login
                    Some(jwt_simple::JWTError::TokenHasExpired) => Redirect::to("/logout").into_response(),
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
                }
            }

            _ => {
                let status = match self {
                    Self::EmailTaken => StatusCode::BAD_REQUEST,
                    Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
                    Self::AssetDoesNotExist | Self::UserDoesNotExist => StatusCode::NOT_FOUND,
                    Self::MissingAuthorization => unreachable!(),
                    Self::Database(_) | Self::Template(_) | Self::Jwt(_) => StatusCode::INTERNAL_SERVER_ERROR
                };

                (status, Json(error_response)).into_response()
            }
        }
    }
}
