pub mod user {
    use serde::{Deserialize, Serialize};
    use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

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
        pub async fn find_one_by_email(
            db: &Surreal<Client>,
            email_id: &str,
        ) -> Result<Option<Self>, surrealdb::Error> {
            let sql = "SELECT * FROM user WHERE email_id = $email";

            let mut result = db.query(sql).bind(("email", email_id)).await?;

            let entries: Vec<UserRecord> = result.take(0)?;

            if entries.is_empty() {
                Ok(None)
            } else {
                Ok(Some(entries[0].to_owned()))
            }
        }
        pub async fn create(
            db: &Surreal<Client>,
            new_user: UserCreation,
        ) -> Result<Option<UserRecord>, surrealdb::Error> {
            let db_response: Result<Option<UserRecord>, surrealdb::Error> = db
                .create(("user", new_user.username.clone()))
                .content(new_user)
                .await;
            db_response
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
