use surrealdb::{engine::remote::ws::Client, Surreal};

use std::sync::Mutex;
pub struct AppContext {
    pub db: Mutex<Surreal<Client>>,
}
