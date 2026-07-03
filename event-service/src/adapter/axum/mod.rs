use std::sync::{Arc, Mutex};
use axum::extract::State;
use axum::Json;
use crate::{ApplicationState, CreateEventInput, DeleteEventInput, Event, EventUseCase};

pub async fn create_event(
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

pub async fn get_events(State(state): State<Arc<Mutex<ApplicationState>>>) -> Json<Vec<Event>> {
    let events = &state.lock().unwrap().event_service.get_all();
    Json(events.clone())
}

pub async fn delete_event(
    State(state): State<Arc<Mutex<ApplicationState>>>,
    Json(input): Json<DeleteEventInput>,
) {
    let _ = &mut state.lock().unwrap().event_service.delete(input.id);
}