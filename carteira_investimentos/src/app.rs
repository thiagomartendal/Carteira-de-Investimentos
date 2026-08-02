use axum::Router;
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};
use tower_http::services::ServeDir;

use crate::modules::account::routes::router as account_router;
use crate::modules::assets::routes::router as assets_router;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub key: Key // Chave secreta para PrivateCookieJar
}

impl AppState {
    async fn new() -> color_eyre::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let db = PgPool::connect(&database_url).await?;
        let key = Key::generate(); // Gera o segredo para PrivateCookieJar

        Ok(Self { db, key })
    }
}

// Implementação para hablitar o uso das chaves para o PrivateCookieJar
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

// Implementação para utilizar State(pool): State<PgPool> na assinatura das funções dos handlers
impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

pub struct App;

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();

        dotenvy::dotenv()?;
        let state = AppState::new().await?;

        let listener = TcpListener::bind("0.0.0.0:3000").await?;
        let router = Router::new()
            .merge(account_router())
            .merge(assets_router())
            .nest_service("/static", ServeDir::new("static"))
            .with_state(state);

        info!("Servidor executando na porta 3000");

        axum::serve(listener, router).await?;

        Ok(())
    }
}
