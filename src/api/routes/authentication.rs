use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

mod route_sign_in {
    #[derive(super::Serialize, super::Deserialize, Debug)]
    pub struct SignInBody {
        email_id: String,
        password: String,
    }
}

mod route_sign_up {
    #[derive(super::Serialize, super::Deserialize, Debug)]
    pub struct SignUpBody {
        username: String,
        email_id: String,
        password: String,
        first_name: String,
        last_name: String,
        age: i8,
        gender: bool,
    }
}

#[post("/signin")]
pub async fn sign_in(body: web::Json<route_sign_in::SignInBody>) -> HttpResponse {
    dbg!(body);
    HttpResponse::Ok().body("SignIn")
}

#[post("/signup")]
pub async fn sign_up(body: web::Json<route_sign_up::SignUpBody>) -> HttpResponse {
    dbg!(body);
    HttpResponse::Ok().body("SignUp")
}
