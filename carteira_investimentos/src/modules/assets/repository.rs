use sqlx::PgPool;

use crate::modules::assets::models::{Asset, OwnedAsset};

pub struct Repository {
    db: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            db: pool
        }
    }

    pub async fn list_assets(&self) -> sqlx::Result<Vec<Asset>> {
        sqlx::query_as!(Asset, "select * from assets")
            .fetch_all(&self.db)
            .await
    }

    pub async fn list_registrant_assets(&self, registrant_id: i64) -> sqlx::Result<Vec<Asset>> {
        sqlx::query_as!(
                Asset,
                "select * from assets where registrant_id = $1;",
                registrant_id
            )
            .fetch_all(&self.db)
            .await
    }

    pub async fn create_asset(&self, name: String, unit_value: f64, registrant_id: i64) -> sqlx::Result<Asset> {
        sqlx::query_as!(
            Asset,
            "insert into assets (name, unit_value, registrant_id) values ($1, $2, $3) returning id, name, unit_value, registrant_id, registered_at;",
            name,
            unit_value,
            registrant_id
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn update_asset(
        &self,
        asset_id: i64,
        name: Option<String>,
        unit_value: Option<f64>,
        registrant_id: i64
    ) -> sqlx::Result<Option<Asset>> {
        sqlx::query_as!(
            Asset,
            "update assets set
            name=COALESCE($2, name),
            unit_value=COALESCE($3, unit_value)
            where id=$1 and registrant_id=$4
            returning id, name, unit_value, registrant_id, registered_at;",
            asset_id,
            name,
            unit_value,
            registrant_id
        )
        .fetch_optional(&self.db)
        .await
    }

    pub async fn delete_asset(&self, asset_id: i64, registrant_id: i64) -> sqlx::Result<()> {
        sqlx::query_as!(
            Asset,
            "delete from assets where id = $1 and registrant_id = $2;",
            asset_id,
            registrant_id
        ).execute(&self.db).await?;

        Ok(())
    }

    pub async fn list_owned_assets(&self, user_id: i64) -> sqlx::Result<Vec<OwnedAsset>> {
        sqlx::query_as!(
            OwnedAsset,
            r#"
            select a.id, a.name, a.unit_value,
            sum((a.unit_value - o.bought_for) * o.quantity_owned) as "value_delta!",
            sum(o.quantity_owned) as "quantity_owned!", 
            json_agg(
                json_build_object(
                    'bought_at', o.timestamp,
                    'bought_for', o.bought_for,
                    'quantity_bought', o.quantity_owned,
                    'value_delta', (a.unit_value - o.bought_for) * o.quantity_owned
                )
            ) as "purchase_history!: _"
            from assets as a
            join owned_assets as o
            on o.asset_id = a.id
            where o.user_id = $1
            group by a.id;
            "#,
            user_id
        ).fetch_all(&self.db).await
    }

    pub async fn insert_owned_asset(
        &self,
        user_id: i64,
        asset_id: i64,
        quantity: f64,
        unit_value: f64
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "insert into owned_assets (user_id, asset_id, quantity_owned, bought_for)
            values ($1, $2, $3, $4);",
            user_id,
            asset_id,
            quantity,
            unit_value
        ).execute(&self.db).await?;

        Ok(())
    }
}

#[cfg(test)]
impl From<PgPool> for Repository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
