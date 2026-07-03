use crate::{Event, EventRepository};

pub struct EventRepositoryInMemory {
    events: Vec<Event>,
}

impl EventRepositoryInMemory {
    pub fn new() -> Self {
        Self { events: vec![]}
    }
}

impl EventRepository for EventRepositoryInMemory {
    fn save(&mut self, event: &Event) -> Result<(), String> {
        self.events.push(event.clone());
        Ok(())
    }

    fn delete(&mut self, id: i128) -> Result<(), String> {
        self.events.retain(|event| event.id != id);
        Ok(())
    }

    fn query_all(&self) -> Vec<Event> {
        self.events.clone()
    }
}