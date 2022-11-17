use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Reqwest: {0}")]
    Reqwest(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Reqwest("test".to_string())
    }
}
