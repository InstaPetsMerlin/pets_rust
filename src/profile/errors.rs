use core::fmt;
use std::error;

impl error::Error for ProfileError {}

#[derive(Debug)]
pub enum ProfileError {
    ProfileNotFoundError(String),
    ProfileDBError(String),
    Other(String),
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProfileError::ProfileNotFoundError(s) => write!(f, "Profile Not Found Error: {}", s),
            ProfileError::Other(s) => write!(f, "Other Error!: {}", s),
            ProfileError::ProfileDBError(s) => write!(f, "Database Error: {}", s),
        }
    }
}
