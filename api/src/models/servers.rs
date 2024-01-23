use super::Event;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Server {
    pub uuid: String,
    pub ip: String,
    pub port: u16,
    pub rcon_password: String,
    pub kills: u32,
    pub deaths: u32,
    pub players: u32,
    pub max_players: u32,
    pub act_map: String,
    pub name: Option<String>,
}


pub struct ServerAmendDeathsEvent {
    pub uuid: String,
    pub deaths: u32,
}

pub struct ServerAmendActMapEvent {
    pub uuid: String,
    pub act_map: String,
}

pub struct ServerAmendRconPasswordEvent {
    pub uuid: String,
    pub rcon_password: String,
}

pub struct ServerAmendIpEvent {
    pub uuid: String,
    pub ip: String,
}

pub struct ServerAmendPortEvent {
    pub uuid: String,
    pub port: u16,
}

pub struct ServerDeleteEvent {
    pub uuid: String,
}
