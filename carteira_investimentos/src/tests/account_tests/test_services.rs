#[cfg(test)]
mod tests {
    use axum_extra::extract::cookie::SameSite;
    use sqlx::PgPool;
    use time::Duration;

    use crate::error::AppError;
    use crate::modules::auth::user::User;
    use crate::modules::account::repository::Repository;
    use crate::modules::account::services; 

    #[sqlx::test]
    async fn test_register_service(db: PgPool) {
        let repository = Repository::new(db.clone());

        let cookie1 = services::register_account(
            &repository,
            String::from("us1@email.com"),
            String::from("Usuário 1"),
            String::from("abcd1234"),
            1
        ).await.expect("");

        let cookie2 = services::register_account(
            &repository,
            String::from("us2@email.com"),
            String::from("Usuário 2"),
            String::from("1234abcd"),
            2
        ).await.expect("");

        let user1 = User::from_auth_token(cookie1.value()).expect("");
        let user2 = User::from_auth_token(cookie2.value()).expect("");

        assert_eq!(cookie1.name(), String::from("token"));
        assert_eq!(cookie1.http_only(), Some(true));
        assert_eq!(cookie1.path(), Some("/"));
        assert_eq!(cookie1.same_site(), Some(SameSite::Strict));
        assert_eq!(cookie1.secure(), Some(true));

        assert_eq!(user1.email, String::from("us1@email.com"));
        assert_eq!(user1.username, String::from("Usuário 1"));
        assert_eq!(user1.user_type, 1);
        
        assert_eq!(cookie2.name(), String::from("token"));
        assert_eq!(cookie2.http_only(), Some(true));
        assert_eq!(cookie2.path(), Some("/"));
        assert_eq!(cookie2.same_site(), Some(SameSite::Strict));
        assert_eq!(cookie2.secure(), Some(true));
        
        assert_eq!(user2.email, String::from("us2@email.com"));
        assert_eq!(user2.username, String::from("Usuário 2"));
        assert_eq!(user2.user_type, 2);
    }

    #[sqlx::test]
    async fn test_login_service(db: PgPool) {
        let repository = Repository::new(db.clone());

        let _ = services::register_account(&repository,
            String::from("us@email.com"),
            String::from("Usuário"),
            String::from("abcd1234"),
            1
        ).await.expect("");
        
        let cookie1 = services::auth_user(
            &repository,
            String::from("us@email.com"),
            String::from("abcd1234"),
            true // Salva o cookie
        ).await.expect("");
        
        let cookie2 = services::auth_user(
            &repository,
            String::from("us@email.com"),
            String::from("senha_incorreta"),
            false // Não salva o cookie
        ).await.unwrap_err(); // Desencapsula o objeto de erro
        
        let cookie3 = services::auth_user(
            &repository,
            String::from("email_incorreto@email.com"),
            String::from("abcd1234"),
            false
        ).await.unwrap_err();

        let user = User::from_auth_token(cookie1.value()).expect("");

        assert_eq!(cookie1.name(), String::from("token"));
        assert_eq!(cookie1.http_only(), Some(true));
        assert_eq!(cookie1.path(), Some("/"));
        assert_eq!(cookie1.same_site(), Some(SameSite::Strict));
        assert_eq!(cookie1.secure(), Some(true));
        assert_eq!(cookie1.max_age(), Some(Duration::weeks(52)));

        assert_eq!(cookie2.to_string(), AppError::InvalidCredentials.to_string());
        assert_eq!(cookie3.to_string(), AppError::UserDoesNotExist.to_string());

        assert_eq!(user.email, String::from("us@email.com"));
        assert_eq!(user.username, String::from("Usuário"));
        assert_eq!(user.user_type, 1);
    }
}
