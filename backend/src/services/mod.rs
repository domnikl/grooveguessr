use std::fmt::{Display, Formatter};

pub mod lobby;

pub enum Error {
    DbError(diesel::result::Error),
    DbConnectionError(r2d2::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DbError(e) => write!(f, "Database Error: {}", e),
            Error::DbConnectionError(e) => write!(f, "Database Connection Error: {}", e),
        }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::DbError(e)
    }
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error::DbConnectionError(e)
    }
}
