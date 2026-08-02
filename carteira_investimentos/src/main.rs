mod app;
pub mod error;
pub mod modules;
pub mod tests;

use crate::app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
