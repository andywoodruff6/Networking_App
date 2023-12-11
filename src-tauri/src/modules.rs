use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub relationship: Relationship,
    pub email: String,
    pub phone_number: String
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub enum Relationship {
    Friend,
    Work,
    Hobby,
}
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct History {
    pub id: i32,
    pub person_id: i32,
    pub date: String,
    pub topic: String,
    pub contact_platform: String
}