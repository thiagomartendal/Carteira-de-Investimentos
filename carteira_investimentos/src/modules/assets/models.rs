use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use time::OffsetDateTime;

// struct para o Ativo
#[derive(Serialize, Clone)]
pub struct Asset { 
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
    pub registrant_id: i64,
    #[serde(with = "time::serde::iso8601")]
    pub registered_at: OffsetDateTime
}

// struct para histórico de compras
#[derive(Serialize, Deserialize)]
pub struct PurchaseHistory {
    #[serde(with = "time::serde::iso8601")]
    pub bought_at: OffsetDateTime,
    pub bought_for: f64,
    pub quantity_bought: f64,
    pub value_delta: f64
}

// struct para ativo comprado
#[derive(Serialize)]
pub struct OwnedAsset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
    pub value_delta: f64,
    pub quantity_owned: f64,
    pub purchase_history: Json<Vec<PurchaseHistory>>
}
