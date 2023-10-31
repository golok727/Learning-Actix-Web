pub mod user {
    use serde::{Deserialize, Serialize};
    use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

    use crate::errors::AppError;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum Gender {
        Male,
        Female,
        Other,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserCreation {
        pub username: String,
        pub password: String,
        pub email_id: String,
        pub first_name: String,
        pub last_name: String,
        pub age: u8,
        pub gender: Gender,
        pub is_admin: bool,
        pub is_verified: bool,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct UserRecord {
        #[allow(dead_code)]
        pub id: Thing,
        pub username: String,
        pub password: String,
        pub email_id: String,
        pub first_name: String,
        pub last_name: String,
        pub age: u8,
        pub gender: Gender,
        pub is_admin: bool,
        pub is_verified: bool,
    }
    impl UserRecord {
        pub async fn find_one_by_email<'a>(
            db: &'a Surreal<Client>,
            email_id: &'a str,
        ) -> Result<Option<Self>, AppError> {
            let sql = "SELECT * FROM user WHERE email_id = $email";

            let mut result = db
                .query(sql)
                .bind(("email", email_id))
                .await
                .map_err(|err| {
                    let error_message = format!("{}", err);
                    AppError::DatabaseQueryError(Some(error_message));
                })
                .unwrap();

            let entries: Vec<UserRecord> = result
                .take(0)
                .map_err(|err| {
                    let error_message = format!(
                        "Something Went Wrong While Getting Users By Email...\n{}",
                        err
                    );

                    AppError::DatabaseError(Some(error_message));
                })
                .unwrap();

            if entries.is_empty() {
                Ok(None)
            } else {
                Ok(Some(entries[0].to_owned()))
            }
        }
        pub async fn create(
            db: &Surreal<Client>,
            new_user: UserCreation,
        ) -> Result<Option<UserRecord>, AppError> {
            let db_response: Result<Option<UserRecord>, surrealdb::Error> = db
                .create(("user", new_user.username.clone()))
                .content(new_user)
                .await;
            match db_response {
                Ok(created) => Ok(created),
                Err(err) => match err {
                    surrealdb::Error::Api(err) => {
                        let error_message =
                            format!("User with username already exists..\nError: {}", &err);
                        Err(AppError::BadRequest(Some(error_message)))
                    }
                    _ => {
                        let error_message = format!("Something Went Wrong!!!\nError: {}", &err);
                        Err(AppError::InternalServerError(Some(error_message)))
                    }
                },
            }
        }
        #[allow(dead_code)]
        pub async fn get_all_users() {
            // TODO:  TO IMPLEMENT}
        }

        #[allow(dead_code)]
        pub async fn get_user_by_id() {
            // todo:  to implement}
        }
    }
}
