use crate::entities::mortgage::Mortgage;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// State объект.
pub type SharedState = Arc<RwLock<AppState>>;

/// Кэш.
#[derive(Default, Debug)]
pub struct AppState {
    pub id: u32,
    pub cache: HashMap<u32, Mortgage>,
}

/// Добавление в кэш.
pub async fn insert(state: Arc<RwLock<AppState>>, loan: Mortgage) {
    let mut binding = state.write().unwrap();
    let id: u32 = binding.id;
    binding.cache.insert(id, loan);
    binding.id += 1;
}
