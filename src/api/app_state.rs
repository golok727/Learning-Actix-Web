use crate::api;
use std::sync::{Arc, Mutex};
pub struct AppState {
    pub app_name: String,
    pub users: Arc<Mutex<Vec<api::user::User>>>,
}
