use std::env;
use std::fs;

#[derive(Debug, Clone)]
pub struct Config {
  amqp_uri: String,
  boot_channel: String,
  boot_routing_key: String,
  bridge_channels: String,
  delivery_mode: u8,
  postgresql_uri: String,
}

fn read_env_with_secret(key: &str) -> String {

  match env::var(format!("{}_FILE", key)) {
    Ok(val) => fs::read_to_string(val.clone()).expect(format!("Something went wrong reading {}", val).as_ref()),
    Err(_e) => env::var(key).expect(format!("{} environment variable must be defined", key).as_ref()),
  }
}

impl Config {

  pub fn new() -> Config {

    Config {
      postgresql_uri: read_env_with_secret("POSTGRESQL_URI"),
      amqp_uri: read_env_with_secret("AMQP_URI"),
      boot_channel: env::var("BOOT_CHANNEL").expect("BOOT_CHANNEL environment variable must be defined"),
      boot_routing_key: env::var("BOOT_ROUTING_KEY").expect("BOOT_CHANNEL environment variable must be defined"),
      bridge_channels: env::var("BRIDGE_CHANNELS").expect("BRIDGE_CHANNELS environment variable must be defined"),
      delivery_mode:
        match env::var("DELIVERY_MODE").ok().as_ref().map(String::as_ref){
          None => 1,
          Some("NON-PERSISTENT") => 1,
          Some("PERSISTENT") => 2,
          Some(_) => panic!("DELIVERY_MODE environment variable can only be PERSISTENT or NON-PERSISTENT")
        }
    }
  }

  pub fn boot_channel(&self) -> &String {
    &self.boot_channel
  }

  pub fn boot_routing_key(&self) -> &String {
    &self.boot_routing_key
  }

  pub fn postgresql_uri(&self) -> &String {
    &self.postgresql_uri
  }

  pub fn amqp_uri(&self) -> &String {
    &self.amqp_uri
  }

  pub fn bridge_channels(&self) -> &String {
    &self.bridge_channels
  }

  pub fn delivery_mode(&self) -> u8 {
    self.delivery_mode
  }

}
