use rocket::serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Tweet {
    pub id: i64,
    pub name: String,
    pub description: String,
}