use axum::extract::State;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let repo = EventRepositoryInMemory { events: vec![] };
    let event_service = CreateEventService { repo };

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
    event_service: CreateEventService<EventRepositoryInMemory>,
}

async fn create_event(
    State(state): State<Arc<Mutex<ApplicationState>>>,
    Json(input): Json<CreateEventInput>,
) {
    let event = Event {
        id: input.id,
        name: input.name,
    };
    let _ = &mut state
        .lock()
        .unwrap()
        .event_service
        .create(event.id, event.name);
}

async fn get_events(State(state): State<Arc<Mutex<ApplicationState>>>) -> Json<Vec<Event>> {
    let events = &state.lock().unwrap().event_service.get_all();
    Json(events.clone())
}

async fn delete_event(
    State(state): State<Arc<Mutex<ApplicationState>>>,
    Json(input): Json<DeleteEventInput>,
) {
    let _ = &mut state.lock().unwrap().event_service.delete(input.id);
}

trait CreateEventUseCase: Send + Sync {
    fn create(&mut self, id: i128, name: String) -> Result<Event, String>;
    fn delete(&mut self, id: i128) -> Result<(), String>;
    fn get_all(&self) -> Vec<Event>;
}

struct CreateEventService<R: EventRepository> {
    repo: R,
}

impl<R: EventRepository> CreateEventUseCase for CreateEventService<R> {
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

struct EventRepositoryInMemory {
    events: Vec<Event>,
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
