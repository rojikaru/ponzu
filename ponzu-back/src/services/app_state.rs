use std::sync::Arc;
use mongodm::mongo::Database;

pub struct AppState {
    pub db: Arc<Database>,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        AppState {
            db: Arc::new(db),
        }
    }
}
