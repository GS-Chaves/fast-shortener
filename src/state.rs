use moka::future::Cache;

#[derive(Clone)]
pub struct AppState {
    pub cache: Cache<String, String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(10_000),
        }
    }
}
