#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::modules::account::repository::Repository;

    #[sqlx::test]
    async fn test_add_user(db: PgPool) {
        let repository = Repository::new(db);

        let user1 = repository.add_user("us1@email.com", "Usuário 1", "senha", 1).await.expect("");
        let user2 = repository.add_user("us2@email.com", "Usuário 2", "senha", 2).await.expect("");

        assert_eq!(user1.email, "us1@email.com");
        assert_eq!(user1.username, "Usuário 1");
        assert_eq!(user1.password_hash, "senha");
        assert_eq!(user1.user_type, 1);

        assert_eq!(user2.email, "us2@email.com");
        assert_eq!(user2.username, "Usuário 2");
        assert_eq!(user2.password_hash, "senha");
        assert_eq!(user2.user_type, 2);
    }

    #[sqlx::test]
    async fn test_get_user_by_email(db: PgPool) {
        let repository = Repository::new(db);

        repository.add_user("us@email.com", "Usuário", "senha", 1).await.expect("");
        
        if let Some(user) = repository.get_user_by_email("us@email.com").await.expect("") {
            assert_eq!(user.email, "us@email.com");
            assert_eq!(user.username, "Usuário");
            assert_eq!(user.password_hash, "senha");
            assert_eq!(user.user_type, 1);
        }

        assert!(repository.get_user_by_email("email_errado@email.com").await.expect("").is_none());
    }
}
