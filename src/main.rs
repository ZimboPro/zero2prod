use std::net::TcpListener;

use sqlx::{PgConnection, Connection, PgPool};
use zero2prod::{startup::run, configuration::get_configuration};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration");
    let connection = PgPool::connect(
        &config.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))
        .unwrap_or_else(|_| panic!("Port {} already is use", config.application_port));
    run(listener, connection)?.await
}
