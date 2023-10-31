#[allow(non_snake_case)]
pub mod Select {
    use serde::Serialize;
    use surrealdb::sql::Thing;

    #[derive(Serialize)]
    pub struct WhereIdAndEmail<'a> {
        pub id: Thing,
        pub email: &'a str,
    }
}
