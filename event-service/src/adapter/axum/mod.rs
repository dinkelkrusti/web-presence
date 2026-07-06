use crate::{ApplicationState, CreateEventInput, DeleteEventInput, Event, EventUseCase};
use axum::Json;
use axum::extract::State;
use std::sync::Arc;

pub async fn create_event(
    State(state): State<Arc<ApplicationState>>,
    Json(input): Json<CreateEventInput>,
) {
    let event = Event {
        id: input.id,
        name: input.name,
    };
    let _ = state
        .event_service
        .write().await
        .create(event.id, event.name);
}

pub async fn get_events(State(state): State<Arc<ApplicationState>>) -> Json<Vec<Event>> {
    let events = state
        .event_service
        .write().await
        .get_all().await;
    Json(events.clone())
}

pub async fn delete_event(
    State(state): State<Arc<ApplicationState>>,
    Json(input): Json<DeleteEventInput>,
) {
    let _ = state
        .event_service
        .write().await
        .delete(input.id);
}
