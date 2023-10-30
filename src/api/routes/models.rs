pub mod user {

    use serde::{Deserialize, Serialize};
    use surrealdb::sql::Thing;

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
        pub age: i8,
        pub gender: Gender,
        pub is_admin: bool,
        pub is_verified: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct UserRecord {
        #[allow(dead_code)]
        pub id: Thing,
        pub username: String,
        pub password: String,
        pub email_id: String,
        pub first_name: String,
        pub last_name: String,
        pub age: i8,
        pub gender: Gender,
        pub is_admin: bool,
        pub is_verified: bool,
    }
}
