use std::sync::Arc;
use mongodm::mongo::Database;

pub struct AppState {
    db: Arc<Database>,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        let db = Arc::new(db);
        AppState {
            db,
        }
    }
}
