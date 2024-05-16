use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AppState {
    count: Arc<Mutex<u32>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            count: Arc::new(Mutex::new(0)),
        }
    }
    pub fn message(&self) -> impl Into<String> {
        "Hello!"
    }
    pub fn counter(&self) -> u32 {
        *self.count.lock().unwrap()
    }
    pub fn increment_counter(&mut self) -> u32 {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        *count
    }
}
