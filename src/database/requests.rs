use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TweetRequest {
    pub name: String,
    pub description: String,
}