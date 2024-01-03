use sqlx::SqlitePool;
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

use crate::{config::Config, error::ConfigError, state::AppState};

mod config;
mod error;
mod forms;
mod routes;
mod state;

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

    let pool = SqlitePool::connect("sqlite:database.db")
        .await
        .unwrap_or_else(|err| {
            error!("couldn't connect to database: {err}");
            std::process::exit(1);
        });

    let state = AppState::new(config, pool);

    info!("starting server");

    let address = state.config().address();
    axum::serve(
        tokio::net::TcpListener::bind(state.config().address())
            .await
            .unwrap_or_else(|err| {
                error!("failed to bind to {address}: {err}");
                std::process::exit(1);
            }),
        routes::router(state),
    )
    .await
    .unwrap_or_else(|err| {
        error!("failed to start a server: {err:?}");
        std::process::exit(1);
    })
}
