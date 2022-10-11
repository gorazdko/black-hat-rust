use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Names {
    name_value: String,
}

#[derive(Debug, Clone)]
pub struct Port {
    pub port: u16,
    pub is_open: bool,
}

#[derive(Debug)]
pub struct Subdomain {
    pub name: String,
    pub port: Vec<Port>,
}
