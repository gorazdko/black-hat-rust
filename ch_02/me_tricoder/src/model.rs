use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Subdomain {
    name: String,
    port: Vec<u8>,
}

#[derive(Deserialize, Debug)]
struct Names {
    name_value: String,
}
