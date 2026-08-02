use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterAssetForm {
    pub name: String,
    pub unit_value: f64
}

#[derive(Deserialize)]
pub struct UpdateAssetJson {
    pub asset_id: i64,
    pub new_name: Option<String>,
    pub new_unit_value: Option<f64>
}

#[derive(Deserialize)]
pub struct PurchaseAssetForm {
    pub asset_id: i64,
    pub unit_value: f64,
    pub quantity: f64
}
