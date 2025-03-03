use crate::models::Graph;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub graph: Arc<Mutex<Option<Graph>>>,
}
