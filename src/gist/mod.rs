pub mod fetch;
pub mod identifier;

#[derive(Debug, PartialEq)]
pub struct Gist {
    pub identifier: String,
    pub cargo: String,
    pub main_rs: String,
}
