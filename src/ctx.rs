use std::sync::{Mutex, MutexGuard};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::errors::AppError;
pub struct Context {
    pub db: Mutex<Surreal<Client>>,
}

impl Context {
    pub fn get_db(&self) -> Result<MutexGuard<Surreal<Client>>, AppError> {
        self.db
            .lock()
            .map_err(|_| AppError::InternalServerError(Some("Error Getting Database".to_owned())))
    }
}
