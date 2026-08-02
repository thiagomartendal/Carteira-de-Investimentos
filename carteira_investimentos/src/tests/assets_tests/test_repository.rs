#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::modules::assets::repository::Repository;

    #[sqlx::test(fixtures("user"))]
    async fn test_create_asset(db: PgPool) {
        let repository = Repository::new(db);

        let asset = repository.create_asset(String::from("Bitcoin"), 10.05, 1).await.expect("");

        assert_eq!(asset.name, "Bitcoin");
        assert_eq!(asset.unit_value, 10.05);
        assert_eq!(asset.registrant_id, 1);
    }

    #[sqlx::test(fixtures("user", "assets"))]
    async fn test_list_assets(db: PgPool) {
        let repository = Repository::new(db);

        let assets = repository.list_assets().await.expect("");

        assert_eq!(assets.len(), 3);

        assert_eq!(assets[0].name, "Bitcoin");
        assert_eq!(assets[0].unit_value, 10.05);
        assert_eq!(assets[0].registrant_id, 1);

        assert_eq!(assets[1].name, "Ethereum");
        assert_eq!(assets[1].unit_value, 9.50);
        assert_eq!(assets[1].registrant_id, 1);

        assert_eq!(assets[2].name, "Lite Coin");
        assert_eq!(assets[2].unit_value, 2.75);
        assert_eq!(assets[2].registrant_id, 2);

        insta::assert_json_snapshot!(assets, {
            ".*.registered_at" => "[TIMESTAMP]"
        });
    }

    #[sqlx::test(fixtures("user", "assets"))]
    async fn test_list_registrant_assets(db: PgPool) {
        let repository = Repository::new(db);

        let assets1 = repository.list_registrant_assets(1).await.expect("");
        let assets2 = repository.list_registrant_assets(2).await.expect("");

        assert_eq!(assets1.len(), 2);
        assert_eq!(assets2.len(), 1);

        assert_eq!(assets1[0].name, "Bitcoin");
        assert_eq!(assets1[0].unit_value, 10.05);
        assert_eq!(assets1[0].registrant_id, 1);

        assert_eq!(assets1[1].name, "Ethereum");
        assert_eq!(assets1[1].unit_value, 9.50);
        assert_eq!(assets1[1].registrant_id, 1);

        assert_eq!(assets2[0].name, "Lite Coin");
        assert_eq!(assets2[0].unit_value, 2.75);
        assert_eq!(assets2[0].registrant_id, 2);

        insta::assert_json_snapshot!(assets1, {
            ".*.registered_at" => "[TIMESTAMP]"
        });

        insta::assert_json_snapshot!(assets2, {
            ".*.registered_at" => "[TIMESTAMP]"
        });
    }

    #[sqlx::test(fixtures("user", "assets"))]
    async fn test_update_asset(db: PgPool) {
        let repository = Repository::new(db);

        repository.update_asset(1, None, Some(15.99), 1).await.expect("");
        repository.update_asset(2, Some(String::from("Monero")), Some(7.25), 1).await.expect("");
        repository.update_asset(3, Some(String::from("Tether")), None, 2).await.expect("");

        let assets = repository.list_assets().await.expect("");

        assert_eq!(assets.len(), 3);

        assert_eq!(assets[0].name, "Bitcoin");
        assert_eq!(assets[0].unit_value, 15.99);
        assert_eq!(assets[0].registrant_id, 1);

        assert_eq!(assets[1].name, "Monero");
        assert_eq!(assets[1].unit_value, 7.25);
        assert_eq!(assets[1].registrant_id, 1);

        assert_eq!(assets[2].name, "Tether");
        assert_eq!(assets[2].unit_value, 2.75);
        assert_eq!(assets[2].registrant_id, 2);

        insta::assert_json_snapshot!(assets, {
            ".*.registered_at" => "[TIMESTAMP]"
        });
    }

    #[sqlx::test(fixtures("user", "assets"))]
    async fn test_delete_asset(db: PgPool) {
        let repository = Repository::new(db);

        repository.delete_asset(1, 1).await.expect("");
        repository.delete_asset(2, 1).await.expect("");
        repository.delete_asset(3, 2).await.expect("");

        let assets = repository.list_assets().await.expect("");

        assert_eq!(assets.len(), 0);
    }

    #[sqlx::test(fixtures("user", "assets", "owned_assets"))]
    async fn test_list_owned_assets(db: PgPool) {
        let repository = Repository::new(db);

        let assets = repository.list_owned_assets(3).await.expect("");

        assert_eq!(assets.len(), 1);

        assert_eq!(assets[0].name, "Bitcoin");
        assert_eq!(assets[0].unit_value, 10.05);
        assert_eq!(assets[0].value_delta, 0.0);
        assert_eq!(assets[0].quantity_owned, 5.0);
        assert_eq!(assets[0].purchase_history[0].bought_for, 10.05);
        assert_eq!(assets[0].purchase_history[0].quantity_bought, 5.0);
        assert_eq!(assets[0].purchase_history[0].value_delta, 0.0);

        insta::assert_json_snapshot!(assets, {
            ".*.purchase_history[0].bought_at" => "[TIMESTAMP]"
        });
    }

    #[sqlx::test(fixtures("user", "assets"))]
    async fn test_insert_owned_asset(db: PgPool) {
        let repository = Repository::new(db);

        repository.insert_owned_asset(3, 1, 3.0, 10.05).await.expect("");
        repository.insert_owned_asset(3, 2, 7.0, 9.50).await.expect("");
        repository.insert_owned_asset(3, 3, 4.0, 2.75).await.expect("");

        let assets = repository.list_owned_assets(3).await.expect("");

        assert_eq!(assets.len(), 3);

        assert_eq!(assets[0].name, "Bitcoin");
        assert_eq!(assets[0].unit_value, 10.05);
        assert_eq!(assets[0].value_delta, 0.0);
        assert_eq!(assets[0].quantity_owned, 3.0);
        assert_eq!(assets[0].purchase_history[0].bought_for, 10.05);
        assert_eq!(assets[0].purchase_history[0].quantity_bought, 3.0);
        assert_eq!(assets[0].purchase_history[0].value_delta, 0.0);

        assert_eq!(assets[1].name, "Ethereum");
        assert_eq!(assets[1].unit_value, 9.50);
        assert_eq!(assets[1].value_delta, 0.0);
        assert_eq!(assets[1].quantity_owned, 7.0);
        assert_eq!(assets[1].purchase_history[0].bought_for, 9.50);
        assert_eq!(assets[1].purchase_history[0].quantity_bought, 7.0);
        assert_eq!(assets[1].purchase_history[0].value_delta, 0.0);

        assert_eq!(assets[2].name, "Lite Coin");
        assert_eq!(assets[2].unit_value, 2.75);
        assert_eq!(assets[2].value_delta, 0.0);
        assert_eq!(assets[2].quantity_owned, 4.0);
        assert_eq!(assets[2].purchase_history[0].bought_for, 2.75);
        assert_eq!(assets[2].purchase_history[0].quantity_bought, 4.0);
        assert_eq!(assets[2].purchase_history[0].value_delta, 0.0);

        insta::assert_json_snapshot!(assets, {
            ".*.purchase_history[0].bought_at" => "[TIMESTAMP]"
        });
    }

    #[sqlx::test(fixtures("user", "assets", "owned_assets"))]
    async fn test_value_delta(db: PgPool) {
        let repository = Repository::new(db);
        
        let assets_before = repository.list_owned_assets(3).await.expect("");

        repository.update_asset(1, None, Some(15.99), 1).await.expect("");

        let assets_after = repository.list_owned_assets(3).await.expect("");

        // Arredonda os valores delta para duas casas decimais
        let rounded_delta_1 = (assets_after[0].value_delta * 100.0).round() / 100.0;
        let rounded_delta_2 = (assets_after[0].purchase_history[0].value_delta * 100.0).round() / 100.0;

        assert_eq!(assets_before[0].name, "Bitcoin");
        assert_eq!(assets_before[0].unit_value, 10.05);
        assert_eq!(assets_before[0].value_delta, 0.0);
        assert_eq!(assets_before[0].quantity_owned, 5.0);
        assert_eq!(assets_before[0].purchase_history[0].bought_for, 10.05);
        assert_eq!(assets_before[0].purchase_history[0].quantity_bought, 5.0);
        assert_eq!(assets_before[0].purchase_history[0].value_delta, 0.0);

        assert_eq!(assets_after[0].name, "Bitcoin");
        assert_eq!(assets_after[0].unit_value, 15.99);
        assert_eq!(rounded_delta_1, 29.70);
        assert_eq!(assets_after[0].quantity_owned, 5.0);
        assert_eq!(assets_after[0].purchase_history[0].bought_for, 10.05);
        assert_eq!(assets_after[0].purchase_history[0].quantity_bought, 5.0);
        assert_eq!(rounded_delta_2, 29.70); 
    }
}
