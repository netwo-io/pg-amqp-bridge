use std::env;
use std::fs;

#[derive(Debug, Clone)]
pub struct Config {
  postgresql_uri: String,
  amqp_uri: String,
  bridge_channels: String,
  delivery_mode: u8,
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

  pub fn get_postgresql_uri(&self) -> &String {
    return &self.postgresql_uri;
  }

  pub fn get_amqp_uri(&self) -> &String {
    return &self.amqp_uri;
  }

  pub fn get_bridge_channels(&self) -> &String {
    return &self.bridge_channels;
  }

  pub fn get_delivery_mode(&self) -> u8 {
    return self.delivery_mode;
  }

}
