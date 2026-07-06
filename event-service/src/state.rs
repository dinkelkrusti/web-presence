use crate::EventService;
use crate::adapter::memory::EventRepositoryInMemory;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct ApplicationState {
    pub(crate) event_repository: Injectable<EventRepositoryInMemory>,
    pub(crate) event_service: Injectable<EventService>,
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