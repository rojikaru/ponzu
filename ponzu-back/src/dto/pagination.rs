/// Pagination struct for returning paginated data
pub struct Pagination<T> {
    pub current_page: u64,
    pub last_page: u64,
    pub per_page: u64,
    pub total: u64,
    pub payload: Vec<T>,
}
