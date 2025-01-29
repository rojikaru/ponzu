use colored::Colorize;
use mongodb::Database;

/// The application state.
pub struct AppState {
}

impl AppState {
    pub fn new(db: Database) -> Self {
        println!("{} {}!", "Initialized AppState with database".bright_green(), db.name());
        AppState {
        }
    }
}
