use crate::{ApplicationState, Event, EventUseCase};
use axum::extract::State;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub async fn start_server(state: Arc<ApplicationState>) {
    let app = Router::new()
        .route("/events", post(create_event))
        .route("/events", get(get_events))
        .route("/events", delete(delete_event))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventInput {
    pub id: i128,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEventInput {
    pub id: i128,
}