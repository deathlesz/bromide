use tracing::{error, info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::error::ConfigError;

mod config;
mod error;
mod routes;

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

    let config = config::Config::try_load().unwrap_or_else(|err| {
        warn!("{err}");

        match err {
            ConfigError::FailedToDeserializeConfig(_) => {
                warn!("delete Config.toml to regenerate config with default values or fix the error above");

                std::process::exit(1);
            }
            _ => {
                info!("generating default config");

                let config = config::Config::default();
                config.try_save().unwrap_or_else(|err| {
                    error!("{err}");
                    warn!("server will continue to start up with default config, however, config will not be saved")
                });
                config
            }
        }
    });

    info!("starting server");

    let address = config.address();
    axum::serve(
        tokio::net::TcpListener::bind(config.address())
            .await
            .unwrap_or_else(|err| {
                error!("failed to bind to {address}: {err}");
                std::process::exit(1);
            }),
        routes::router(config),
    )
    .await
    .unwrap_or_else(|err| {
        error!("failed to start a server: {err:?}");
        std::process::exit(1);
    })
}
