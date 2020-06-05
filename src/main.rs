#[macro_use] extern crate log;
mod config;
mod pg_model;
mod bridge;

use crate::config::Config;

use std::thread;
use std::time::Duration;
use r2d2::{Pool, ManageConnection};
use r2d2_postgres::{TlsMode, PostgresConnectionManager};

fn main() {
  env_logger::init().unwrap();
  let config = Config::new();

  let pool = wait_for_pg_connection(config.postgresql_uri());
  bridge::boot(pool.get().unwrap(), config.amqp_uri().to_string(), config.boot_channel().to_string(), config.boot_routing_key().to_string(), config.delivery_mode())
    .join().unwrap();

  loop {
    let pool = wait_for_pg_connection(&config.postgresql_uri());
    // This functions spawns threads for each pg channel and waits for the threads to finish,
    // that only occurs when the threads die due to a pg connection error
    // and so if that happens the pg connection is retried and the bridge is started again.
    bridge::start_consumers(pool, &config.amqp_uri(), &config.bridge_channels(), config.delivery_mode());
  }
}

fn wait_for_pg_connection(pg_uri: &str) -> Pool<PostgresConnectionManager> {

  println!("Attempting to connect to PostgreSQL..");
  let conn = PostgresConnectionManager::new(pg_uri.to_owned(), TlsMode::None).unwrap();
  let mut i = 1;
  while let Err(e) = conn.connect() {
    println!("{:?}", e);
    let time = Duration::from_secs(i);
    println!("Retrying the PostgreSQL connection in {:?} seconds..", time.as_secs());
    thread::sleep(time);
    i *= 2;
    if i > 32 { i = 1 };
  };
  println!("Connection to PostgreSQL successful");
  Pool::new(conn).unwrap()
}
