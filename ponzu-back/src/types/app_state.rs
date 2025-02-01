use mongodb::Database;

/// The application state.
pub struct AppState {
}

impl AppState {
    pub fn new(db: Database) -> Self {
        AppState {
        }
    }
}
