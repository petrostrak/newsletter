use std::net::TcpListener;

use env_logger::Env;
use newsletter::{configuration::get_configuration, startup::run};
use sqlx::PgPool;
use tracing::dispatcher::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("information"));

    let formatiting_layer = BunyanFormattingLayer::new("newsletter".into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatiting_layer);

    set_global_default(subscriber.into()).expect("Failed to set subscriber");

    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to localhost.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres ");

    run(listener, connection_pool)?.await
}
