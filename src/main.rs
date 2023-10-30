use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Radhey Shyam");

    let state = web::Data::new(api::app_state::AppState {
        app_name: String::from("Radha Krsna"),
        users: Arc::new(Mutex::new(api::user::make_users())),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(api::user::users_route)
            .service(api::user::signup_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
