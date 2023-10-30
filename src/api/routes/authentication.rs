use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::utils;

use super::models;

mod route_sign_in {
    #[derive(super::Serialize, super::Deserialize, Debug)]
    pub struct SignInBody {
        pub email_id: String,
        pub password: String,
    }
}

mod route_sign_up {
    use crate::api::routes::models::user;

    #[derive(super::Serialize, super::Deserialize, Debug)]
    pub struct SignUpBody {
        pub username: String,
        pub email_id: String,
        pub password: String,
        pub first_name: String,
        pub last_name: String,
        pub age: i8,
        pub gender: user::Gender,
    }
}

#[post("/signin")]
pub async fn sign_in(body: web::Json<route_sign_in::SignInBody>) -> HttpResponse {
    dbg!(body);
    HttpResponse::Ok().body("SignIn")
}

#[post("/signup")]
pub async fn sign_up(
    body: web::Json<route_sign_up::SignUpBody>,
    db: web::Data<Surreal<Client>>,
) -> HttpResponse {
    dbg!(&body);

    let username = &body.username;
    let email_id = &body.email_id;
    let password = &body.password;
    let first_name = &body.first_name;
    let last_name = &body.last_name;
    let gender: &models::user::Gender = &body.gender;

    // Hash the password
    let hashed_password = utils::password::hash_password(&password)
        .map_err(|_| HttpResponse::InternalServerError().body(format!("Something Went Wrong!...")))
        .unwrap();

    // Create a user in the database
    // Each user should have unique user_name which will be used as user_id
    // Checks for user exits will be made by the db engine itself
    // Todo Think of adding check for unique email;
    let db_response: Result<Option<models::user::UserRecord>, surrealdb::Error> = db
        .create(("user", body.username.clone()))
        .content(models::user::UserCreation {
            username: username.to_string(),
            email_id: email_id.to_string(),
            password: hashed_password,
            age: body.age,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            gender: gender.clone(),
            is_admin: true,
            is_verified: true,
        })
        .await;

    match db_response {
        Ok(created_user) => {
            dbg!("New User Created");
            dbg!(&created_user);

            match created_user {
                Some(user) => HttpResponse::Created().json(user),
                _ => return HttpResponse::Created().body("User created"),
            }
        }
        Err(err) => match err {
            surrealdb::Error::Api(err) => HttpResponse::BadRequest().body(format!(
                "User with username already exists..\nError: {}",
                &err
            )),
            _ => HttpResponse::InternalServerError()
                .body(format!("Something Went Wrong!!!\nError: {}", err)),
        },
    }
}
