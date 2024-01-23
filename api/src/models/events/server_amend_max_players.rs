use serde::Serialize;
#[derive(Debug, Serialize, Clone)]

pub struct Inner {
  pub uuid: String,
  pub max_players: u32,
}