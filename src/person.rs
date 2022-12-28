use serde::Deserialize;

#[derive(Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
}