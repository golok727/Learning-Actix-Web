use crate::api::app_state;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    username: String,
    age: i8,
    email: String,
}

impl User {
    pub fn new(username: &str, age: i8, email: &str) -> Self {
        User {
            username: username.to_string(),
            age,
            email: email.to_string(),
        }
    }
}

pub fn make_users() -> Vec<User> {
    let u1 = User::new("Radha", 19, "radhakrsna@golok.vrindavan");
    let u2 = User::new("Krsna", 19, "krsnaradha@golok.vrindavan");
    let u3 = User::new("Adithyan", 19, "aadi@golok.vrindavan");
    let u4 = User::new("Mahadev", 19, "mahadev@golok.vrindavan");

    let users_list: Vec<User> = vec![u1, u2, u3, u4];
    users_list
}

#[post("/signup")]
pub async fn signup_route(
    body: web::Json<User>,
    state: web::Data<app_state::AppState>,
) -> Result<impl Responder> {
    let mut state_users = state.users.lock().unwrap();
    let db_users = state_users.deref_mut();

    let username = body.username.clone();
    let age = body.age;
    let email = body.email.clone();

    if db_users.iter().any(|user| user.email == email) {
        return Err(actix_web::error::ErrorBadRequest(format!(
            "The user with email '{}' already exists",
            &email
        )));
    }

    let new_user = User::new(&username, age, &email);

    db_users.push(new_user.clone());

    Ok(HttpResponse::Ok().json(new_user))
}

#[get("/users")]
pub async fn users_route(state: web::Data<app_state::AppState>) -> Result<impl Responder> {
    let app_name = state.app_name.clone();
    let state_users_mutex = state.users.lock().unwrap();
    let db_users = state_users_mutex.deref();

    println!("App name from state: {}", app_name);

    Ok(HttpResponse::Ok().json(db_users.clone()))
}
