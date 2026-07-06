use crate::adapter::memory::EventRepositoryInMemory;
use crate::port::driven::EventRepository;
use crate::port::driver::EventUseCase;
use crate::state::Injectable;
use serde::{Deserialize, Serialize};

pub struct EventService {
    pub repo: Injectable<EventRepositoryInMemory>
}

impl EventUseCase for EventService {
    async fn create(&mut self, id: i128, name: String) -> Result<Event, String> {
        let event = Event { id, name };

        match self.repo.write().await.save(&event) {
            Ok(_) => Ok(event),
            Err(e) => Err(e.clone()),
        }
    }

    async fn delete(&mut self, id: i128) -> Result<(), String> {
        self.repo.write().await.delete(id)
    }

    async fn get_all(&self) -> Vec<Event> {
        self.repo.read().await.query_all()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: i128,
    pub name: String,
}