use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Reqwest: {0}")]
    Reqwest(String),
}
