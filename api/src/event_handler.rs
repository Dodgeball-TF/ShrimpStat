use macro_utils::UnclassifiedEvent;

pub trait Event {
  fn handle(&self);
}

#[UnclassifiedEvent]
pub enum UnclassifiedEvent{
  ServerAmendActMap,
  ServerAmendDeaths,
  ServerAmendIp,
  ServerAmendKills,
  ServerAmendMaxPlayers,
  ServerAmendName,
  ServerAmendPlayers,
  ServerAmendPort,
  ServerAmendRconPassword,
  ServerDelete,
  ServerRegister
}

