use crate::logic::mortgage::Mortgage;
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
pub async fn insert(state: Arc<RwLock<AppState>>, loan: Mortgage) -> u32 {
    let mut binding = state.write().unwrap();
    let id: u32 = binding.id;
    binding.cache.insert(id, loan);
    binding.id += 1;
    id
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::mortgage::Program;
    use crate::schema::Request;

    #[tokio::test]
    async fn test_insert() {
        let req = Request {
            object_cost: 100.0,
            initial_payment: 30.0,
            months: 12,
            program: Program {
                base: Some(true),
                military: None,
                salary: None,
            },
        };
        let state: Arc<RwLock<AppState>> = SharedState::default();
        let loan: Mortgage = Mortgage::new(req);

        let actual: u32 = insert(state, loan).await;
        assert_eq!(actual, 0)
    }
}
