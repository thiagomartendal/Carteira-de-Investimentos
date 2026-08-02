#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::modules::assets::repository::Repository;
    use crate::modules::assets::services;

    #[sqlx::test(fixtures("user"))]
    async fn test_register_asset(db: PgPool) {
        let repository = Repository::new(db);
        
        let status = services::register_asset(&repository, String::from("Ativo 1"), 1.5, 1).await.expect("");

        assert_eq!(status, ());
    }
    
    #[sqlx::test(fixtures("user", "assets"))]
    async fn test_update_asset(db: PgPool) {
        let repository = Repository::new(db);

        let status1 = services::update_asset(&repository, 1, None, Some(15.99), 1).await.expect("");
        let status2 = services::update_asset(&repository, 2, Some(String::from("Monero")), None, 1).await.expect("");
        let status3 = services::update_asset(&repository, 3, Some(String::from("Tether")), Some(3.25), 2).await.expect("");

        assert_eq!(status1, ());
        assert_eq!(status2, ());
        assert_eq!(status3, ());
    }

    #[sqlx::test(fixtures("user", "assets"))]
    async fn test_delete_asset(db: PgPool) {
        let repository = Repository::new(db);

        let status1 = services::delete_asset(&repository, 1, 1).await.expect("");
        let status2 = services::delete_asset(&repository, 2, 1).await.expect("");
        let status3 = services::delete_asset(&repository, 3, 2).await.expect("");

        assert_eq!(status1, ());
        assert_eq!(status2, ());
        assert_eq!(status3, ());
    }

    #[sqlx::test(fixtures("user", "assets"))]
    async fn test_purchase_asset(db: PgPool) {
        let repository = Repository::new(db);

        let status = services::purchase_asset(&repository, 1, 5.0, 10.05, 3).await.expect("");

        assert_eq!(status, ());
    }

    #[sqlx::test(fixtures("user", "assets", "owned_assets"))]
    async fn test_assets_page(db: PgPool) {
        let repository = Repository::new(db);

        // Faz mais uma compra
        services::purchase_asset(&repository, 2, 7.0, 9.50, 3).await.expect("");

        let (available_assets_1, owned_assets_1, wallet_value_1) = services::assets_page(&repository, 3).await.expect("");

        // Atualiza o valor do ativo para testar a variação
        services::update_asset(&repository, 1, None, Some(15.99), 1).await.expect("");
        services::update_asset(&repository, 2, None, Some(10.35), 1).await.expect("");

        let (available_assets_2, owned_assets_2, wallet_value_2) = services::assets_page(&repository, 3).await.expect("");
        let rounded_wallet_value = (wallet_value_2 * 100.0).round() / 100.0;

        assert_eq!(wallet_value_1, 0.0);
        assert_eq!(rounded_wallet_value, 35.65);

        insta::assert_json_snapshot!(available_assets_1, {
            ".*.registered_at" => "[TIMESTAMP]"
        });

        insta::assert_json_snapshot!(owned_assets_1, {
            ".*.purchase_history[0].bought_at" => "[TIMESTAMP]"
        });

        insta::assert_json_snapshot!(available_assets_2, {
            ".*.registered_at" => "[TIMESTAMP]"
        });

        insta::assert_json_snapshot!(owned_assets_2, {
            ".*.purchase_history[0].bought_at" => "[TIMESTAMP]"
        });
    }

    #[sqlx::test(fixtures("user", "assets"))]
    async fn test_manage_page(db: PgPool) {
        let repository = Repository::new(db);

        let assets1 = services::manage_page(&repository, 1).await.expect("");
        let assets2 = services::manage_page(&repository, 2).await.expect("");

        insta::assert_json_snapshot!(assets1, {
            ".*.registered_at" => "[TIMESTAMP]"
        });

        insta::assert_json_snapshot!(assets2, {
            ".*.registered_at" => "[TIMESTAMP]"
        });
    }
}
