use crate::Event;

pub trait EventUseCase: Send + Sync {
    async fn create(&mut self, id: i128, name: String) -> Result<Event, String>;
    async fn delete(&mut self, id: i128) -> Result<(), String>;
    async fn get_all(&self) -> Vec<Event>;
}