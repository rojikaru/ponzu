use mongodb::Database;
use std::sync::Arc;

pub struct AppState {
    db: Arc<Database>,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        let db = Arc::new(db);
        AppState { db }
    }
}
