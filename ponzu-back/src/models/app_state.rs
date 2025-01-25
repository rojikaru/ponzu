use mongodb::Database;
use std::sync::Arc;

/// The application state.
pub struct AppState {
    /// The MongoDB database that the application uses.
    pub(crate) db: Arc<Database>,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        let db = Arc::new(db);
        AppState { db }
    }
}
