mod adapter;

use crate::adapter::axum::{create_event, delete_event, get_events};
use crate::adapter::memory::EventRepositoryInMemory;
use axum::Router;
use axum::routing::{delete, get, post};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[tokio::main]
async fn main() {
    let event_repository = Injectable::new(EventRepositoryInMemory::new());
    let event_service =Injectable::new( EventService { repo: event_repository.clone() });

    let state = Arc::new(ApplicationState { event_service, event_repository });

    let app = Router::new()
        .route("/events", post(create_event))
        .route("/events", get(get_events))
        .route("/events", delete(delete_event))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

struct ApplicationState {
    event_repository: Injectable<EventRepositoryInMemory>,
    event_service: Injectable<EventService>,
}

#[derive(Debug, Clone)]
pub struct Injectable<T>(Arc<RwLock<T>>);

impl<T> Injectable<T> {
    pub fn new(value: T) -> Self {
        Injectable(Arc::new(RwLock::new(value)))
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().await
    }
}

trait EventUseCase: Send + Sync {
    async fn create(&mut self, id: i128, name: String) -> Result<Event, String>;
    async fn delete(&mut self, id: i128) -> Result<(), String>;
    async fn get_all(&self) -> Vec<Event>;
}

struct EventService {
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
