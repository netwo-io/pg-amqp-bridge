#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Type {
  Exchange,
  Queue
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Binding{
  pub pg_channel: String,
  pub amqp_entity: String
}

//Used for channel ids
pub struct ChannelCounter{
  counter: u16
}

impl ChannelCounter {

  pub fn new() -> ChannelCounter {
    ChannelCounter { counter: 0 }
  }

  pub fn inc(&mut self) -> u16 {
    self.counter += 1;
    self.counter
  }
}

pub struct Message {

  pub id: uuid::Uuid,
  pub routing_key: String,
  pub payload: serde_json::Value
}
