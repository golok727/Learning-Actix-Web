use crate::{ctx::Context, errors::AppError};
use actix_web::{post, web, web::Data, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::utils;

use super::models::user;

mod route_sign_in {

    #[derive(super::Serialize, super::Deserialize, Debug)]
    pub struct SignInBody {
        pub email_id: Option<String>,
        pub username: Option<String>,
        pub password: String,
    }
}

mod route_sign_up {
    use crate::api::routes::models::user::{self, UserRecord};

    #[derive(super::Serialize, super::Deserialize, Debug)]
    pub struct SignUpBody {
        pub username: String,
        pub email_id: String,
        pub password: String,
        pub first_name: String,
        pub last_name: String,
        pub age: u8,
        pub gender: user::Gender,
    }

    #[derive(super::Serialize)]
    pub struct UserResponse<'a> {
        pub username: &'a str,
        pub email_id: &'a str,
        pub first_name: &'a str,
        pub last_name: &'a str,
        pub age: u8,
        pub gender: user::Gender,
    }

    #[derive(super::Serialize)]
    pub struct SignUpResponse<'a> {
        pub status: u8,
        pub message: &'a str,
        pub new_user: UserResponse<'a>,
    }

    impl<'a> SignUpResponse<'a> {
        pub fn new(user: &'a UserRecord) -> Self {
            Self {
                status: 201,
                message: "User Created",
                new_user: UserResponse {
                    username: &user.username,
                    email_id: &user.email_id,
                    first_name: &user.first_name,
                    last_name: &user.last_name,
                    age: user.age,
                    gender: user.gender.clone(),
                },
            }
        }
    }
}
#[post("/signin")]
pub async fn sign_in(
    _body: web::Json<route_sign_in::SignInBody>,
    _ctx: Data<Context>,
) -> Result<HttpResponse, AppError> {
    // Allow to sign in with both username or email

    Ok(HttpResponse::Ok().body("SignIn"))
}

#[post("/signup")]
pub async fn sign_up(
    body: web::Json<route_sign_up::SignUpBody>,
    ctx: Data<Context>,
) -> Result<HttpResponse, AppError> {
    let db = ctx.get_db()?;

    let username = &body.username;
    let email_id = &body.email_id;
    let password = &body.password;
    let first_name = &body.first_name;
    let last_name = &body.last_name;
    let gender: &user::Gender = &body.gender;

    // check if email already exists;
    let db_user = user::UserRecord::find_one_by_email(&db, &email_id).await?;

    // Return bad request if the email already exists
    if Option::is_some(&db_user) {
        let error_message = format!(
            "The user with email '{}' or id {} already exists.",
            &email_id, &username
        );
        return Err(AppError::BadRequest(Some(error_message)));
    }

    // Hash the password
    let hashed_password = utils::password::hash_password(&password)?;

    // Create a user in the database
    // Each user should have unique user_name which will be used as user_id
    // Checks for user_id exits will be made by the db engine itself
    let new_user = user::UserCreation {
        username: username.to_string(),
        email_id: email_id.to_string(),
        password: hashed_password,
        age: body.age,
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        gender: gender.clone(),
        is_admin: false,
        is_verified: false,
    };

    // Create a new user
    let created_user = user::UserRecord::create(&db, new_user).await?;

    // If no error send back the user
    // to do make a custom response
    match created_user {
        Some(user) => {
            let response = route_sign_up::SignUpResponse::new(&user);

            Ok(HttpResponse::Created().json(response))
        }
        _ => Err(AppError::DatabaseError(Some(
            "Something Went Wrong".to_owned(),
        ))),
    }
}
