pub mod servers;

pub trait Event {
    fn process(&self) -> String;
    fn save(&self) -> String;
}
