use crate::application::Event;

pub trait EventRepository: Send + Sync {
    fn save(&mut self, event: &Event) -> Result<(), String>;
    fn delete(&mut self, id: i128) -> Result<(), String>;
    fn query_all(&self) -> Vec<Event>;
}