mod adapter;
pub mod state;
pub mod port;
pub mod application;

use crate::adapter::axum::start_server;
use crate::adapter::memory::EventRepositoryInMemory;
use crate::application::{Event, EventService};
use crate::port::driver::EventUseCase;
use crate::state::{ApplicationState, Injectable};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let event_repository = Injectable::new(EventRepositoryInMemory::new());
    let event_service =Injectable::new( EventService { repo: event_repository.clone() });

    let state = Arc::new(ApplicationState { event_service, event_repository });

    start_server(state.clone()).await;
}
