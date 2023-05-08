use std::net::TcpListener;

use once_cell::sync::Lazy;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use uuid::Uuid;
use zero2prod::{telemetry::{get_subscriber, init_subscriber}, configuration::{get_configuration, DatabaseSettings}, email_client::EmailClient, startup::{run, get_connection_pool, Application}};


static TRACING: Lazy<()> = Lazy::new(|| {
  let default_filter_level = "info".to_string();
  let subscriber_name = "test".to_string();

  if std::env::var("TEST_LOG").is_ok() {
      let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
      init_subscriber(subscriber);
  } else {
      let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
      init_subscriber(subscriber);
  }
});

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
  Lazy::force(&TRACING);

  let configuration = {
    let mut c = get_configuration().expect("Failed to read configuration.");
    c.database.database_name = Uuid::new_v4().to_string();
    c.application.port = 0;
    c
  };

  configure_database(&configuration.database).await;

  let application = Application::build(configuration.clone())
    .await
    .expect("Failed to build application.");
  let address = format!("http://127.0.0.1:{}", application.port());
  let _ = tokio::spawn(application.run_until_stopped());
  
  TestApp {
      address,
      db_pool: get_connection_pool(&configuration.database),
  }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
  let mut connection = PgConnection::connect_with(&config.without_db())
      .await
      .expect("Failed to connect to Postgres");
  connection
      .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
      .await
      .expect("Failed to create database.");
  // Migrate database
  let connection_pool = PgPool::connect_with(config.with_db())
      .await
      .expect("Failed to connect to Postgres.");
  sqlx::migrate!("./migrations")
      .run(&connection_pool)
      .await
      .expect("Failed to migrate the database");
  connection_pool
}