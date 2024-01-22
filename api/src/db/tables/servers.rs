use crate::db::tables::Event;
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

#[derive(Debug, Serialize, Clone)]
pub struct ServerRegisterEvent {
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

impl Event for ServerRegisterEvent {
    fn process(&self) -> String {
        // TODO: Add ways to process this event by saving it to database
        return "".to_string();
    }
    fn save(&self) -> String {
        // TODO: create event in master_events by saving this whole object
        return "".to_string();
    }
}

pub struct ServerAmendNameEvent {
    pub uuid: String,
    pub name: String,
}

pub struct ServerAmendPlayersEvent {
    pub uuid: String,
    pub players: u32,
}

pub struct ServerAmendMaxPlayersEvent {
    pub uuid: String,
    pub max_players: u32,
}

pub struct ServerAmendKillsEvent {
    pub uuid: String,
    pub kills: u32,
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
