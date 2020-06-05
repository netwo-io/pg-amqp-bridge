#[macro_use] extern crate log;
mod config;
mod pg_model;
mod bridge;

use crate::config::Config;

use postgres::NoTls;
use std::thread;
use std::time::Duration;
use r2d2::{Pool, ManageConnection};
use r2d2_postgres::PostgresConnectionManager;

fn main() {
  env_logger::init().unwrap();
  let config = Config::new();

  loop {
    let pool = wait_for_pg_connection(&config.get_postgresql_uri());
    // This functions spawns threads for each pg channel and waits for the threads to finish,
    // that only occurs when the threads die due to a pg connection error
    // and so if that happens the pg connection is retried and the bridge is started again.
    bridge::start(pool, &config.get_amqp_uri(), &config.get_bridge_channels(), config.get_delivery_mode());
  }
}

fn wait_for_pg_connection(pg_uri: &String) -> Pool<PostgresConnectionManager<NoTls>> {

  println!("Attempting to connect to PostgreSQL..");
  let conn = PostgresConnectionManager::new(pg_uri.parse().unwrap(), NoTls);
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
  return Pool::new(conn).unwrap();
}
