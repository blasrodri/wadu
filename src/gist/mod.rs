pub mod fetch;
pub mod identifier;

#[derive(Debug, PartialEq)]
pub struct Gist {
    pub identifier: String,
    pub cargo: String,
    pub main_rs: String,
}

impl Gist {
    pub fn new(identifier: String, cargo: String, main_rs: String) -> Self {
        Self {
            identifier,
            cargo,
            main_rs,
        }
    }
}
