#[macro_use] extern crate log;
mod config;
mod pg_model;
mod bridge;

use crate::config::Config;

use r2d2::{Pool, ManageConnection};
use r2d2_postgres::{TlsMode, PostgresConnectionManager};

fn main() {
  env_logger::init().unwrap();
  let config = Config::new();

  let pool = wait_for_pg_connection(&config);
  bridge::boot(pool.get().unwrap(), config.amqp_uri().to_string(), config.boot_channel().to_string(), config.delivery_mode(), config.unacknowledged_bulk_size())
    .join().unwrap();

  loop {
    let pool = wait_for_pg_connection(&config);
    // This functions spawns threads for each pg channel and waits for the threads to finish,
    // that only occurs when the threads die due to a pg connection error
    // and so if that happens the pg connection is retried and the bridge is started again.
    bridge::start_consumers(pool, &config.amqp_uri(), &config.bridge_channels(), config.delivery_mode());
  }
}

fn wait_for_pg_connection(config: &Config) -> Pool<PostgresConnectionManager> {

  println!("Attempting to connect to PostgreSQL..");
  let conn = PostgresConnectionManager::new(config.postgresql_uri().to_owned(), TlsMode::None).unwrap();
  if let Err(e) = conn.connect() {
    panic!("{:?}", e);
  };
  println!("Connection to PostgreSQL successful");
  Pool::builder().max_size(config.pool_size()).build(conn).unwrap()
}
