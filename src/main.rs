use std::hash;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

mod api;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Radhey Shyam");
    // Initialize Logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Start Database
    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .map_err(|err| println!("Error Connecting to DB...\n Error: {}", err))
        .unwrap();

    // Sign Into DataBase
    // Todo use ENV variables
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .map_err(|err| println!("Error Logging Into Database...\n Error: {}", err))
    .unwrap();

    // Choose namespace and database
    db.use_ns("development")
        .use_db("testing")
        .await
        .map_err(|err| println!("Database Connection Error: \n {}", err))
        .unwrap();

    // setup the database in the state

    HttpServer::new(move || {
        let api_scope = web::scope("/api")
            .service(api::routes::authentication::sign_in)
            .service(api::routes::authentication::sign_up);

        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db.clone()))
            .service(api_scope)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
