mod adapter;

use crate::adapter::axum::{create_event, delete_event, get_events};
use crate::adapter::memory::EventRepositoryInMemory;
use axum::routing::{delete, get, post};
use axum::Router;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let repo = EventRepositoryInMemory::new();
    let event_service = EventService { repo };

    let state = Arc::new(Mutex::new(ApplicationState { event_service }));
    let app = Router::new()
        .route("/events", post(create_event))
        .route("/events", get(get_events))
        .route("/events", delete(delete_event))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

struct ApplicationState {
    event_service: EventService<EventRepositoryInMemory>,
}



trait EventUseCase: Send + Sync {
    fn create(&mut self, id: i128, name: String) -> Result<Event, String>;
    fn delete(&mut self, id: i128) -> Result<(), String>;
    fn get_all(&self) -> Vec<Event>;
}

struct EventService<R: EventRepository> {
    repo: R,
}

impl<R: EventRepository> EventUseCase for EventService<R> {
    fn create(&mut self, id: i128, name: String) -> Result<Event, String> {
        let event = Event { id, name };
        match &self.repo.save(&event) {
            Ok(_) => Ok(event),
            Err(e) => Err(e.clone()),
        }
    }

    fn delete(&mut self, id: i128) -> Result<(), String> {
        self.repo.delete(id)
    }

    fn get_all(&self) -> Vec<Event> {
        self.repo.query_all()
    }
}

trait EventRepository: Send + Sync {
    fn save(&mut self, event: &Event) -> Result<(), String>;
    fn delete(&mut self, id: i128) -> Result<(), String>;
    fn query_all(&self) -> Vec<Event>;
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateEventInput {
    pub id: i128,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeleteEventInput {
    pub id: i128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Event {
    pub id: i128,
    pub name: String,
}
