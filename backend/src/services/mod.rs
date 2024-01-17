use std::fmt::{Display, Formatter};

pub mod content;
pub mod lobby;
pub mod presence;
pub mod user;

pub enum Error {
    Db(diesel::result::Error),
    DbConnection(r2d2::Error),
    RedisConnection(redis::RedisError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Db(e) => write!(f, "Database Error: {}", e),
            Error::DbConnection(e) => write!(f, "Database Connection Error: {}", e),
            Error::RedisConnection(e) => write!(f, "Redis Connection Error: {}", e),
        }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::Db(e)
    }
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error::DbConnection(e)
    }
}
