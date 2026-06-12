use axum::{Json, Router};
use axum::routing::post;
use std::sync::{Arc, Mutex};
use axum::extract::State;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let repo = EventRepositoryInMemory { events: vec![] };
    let event_service = CreateEventService { repo };

    let state = Arc::new(Mutex::new(ApplicationState {event_service}));
    let app = Router::new()
        .route("/events", post(create_event))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_event(
    State(state): State<Arc<Mutex<ApplicationState>>>,
    Json(input): Json<CreateEventInput>,
) {
    let event = Event { id: input.id, name: input.name };
    let _ = &mut state.lock().unwrap().event_service.create(event.id, event.name);
}

struct ApplicationState {
    event_service: CreateEventService<EventRepositoryInMemory>,
}

trait CreateEventUseCase: Send + Sync {
    fn create(&mut self, id: i128, name: String) -> Result<Event, String>;
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
}

trait EventRepository: Send + Sync {
    fn save(&mut self, event: &Event) -> Result<(), String>;
}

struct EventRepositoryInMemory {
    events: Vec<Event>,
}

impl EventRepository for EventRepositoryInMemory {
    fn save(&mut self, event: &Event) -> Result<(), String> {
        self.events.push(event.clone());
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateEventInput {
    pub id: i128,
    pub name: String,
}

#[derive(Debug, Clone)]
struct Event {
    pub id: i128,
    pub name: String,
}
