extern crate response_error;

use sqlx::SqlitePool;
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

use crate::error::ConfigError;
pub(crate) use crate::{config::Config, state::AppState};

mod config;
mod error;
mod forms;
mod routes;
mod state;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bromide=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("loading config");
    let config = tokio::task::spawn_blocking(move || {
        Config::try_load().unwrap_or_else(|err| {
            warn!("{err}");

            match err {
                ConfigError::FailedToDeserializeConfig(_) => {
                    warn!("delete Config.toml to regenerate config with default values or fix the error above");

                    std::process::exit(1);
                }
                _ => {
                    info!("generating default config");

                    let config = Config::default();
                    config.try_save().unwrap_or_else(|err| {
                        error!("{err}");
                        warn!("server will continue to start up with default config, however, config will not be saved")
                    });
                    config
                }
            }
        })
    }).await.expect("must not panic");

    info!("connecting to database");
    let pool = SqlitePool::connect(
        &std::env::vars()
            .find(|(name, _)| name == "DATABASE_URL")
            .map(|(_, value)| value)
            .unwrap_or("sqlite:bromide.db".into()),
    )
    .await
    .unwrap_or_else(|err| {
        error!("couldn't connect to database: {err}");
        std::process::exit(1);
    });

    info!("performing migrations");
    sqlx::migrate!().run(&pool).await.unwrap_or_else(|err| {
        error!("failed to migrate: {err}");
        std::process::exit(1);
    });

    let state = AppState { config, pool };

    info!("starting server");
    let address = state.config.address();
    axum::serve(
        tokio::net::TcpListener::bind(state.config.address())
            .await
            .unwrap_or_else(|err| {
                error!("failed to bind to {address}: {err}");
                std::process::exit(1);
            }),
        routes::router(state),
    )
    .with_graceful_shutdown(utils::shutdown_signal())
    .await
    .unwrap_or_else(|err| {
        error!("failed to start a server: {err:?}");
        std::process::exit(1);
    })
}
