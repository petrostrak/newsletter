use secrecy::ExposeSecret;
use std::net::TcpListener;

use newsletter::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to localhost.");
    let connection_pool =
        PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres ");

    run(listener, connection_pool)?.await
}
