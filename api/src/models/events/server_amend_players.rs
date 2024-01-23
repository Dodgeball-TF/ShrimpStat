use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Clone)]
pub struct Inner {
  pub uuid: String,
  pub players: u32,
}