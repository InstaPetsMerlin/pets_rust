use core::fmt;
use std::error;

impl error::Error for PostError {}

#[derive(Debug)]
pub enum PostError {
    PostNotFoundError(String),
    PostDBError(String),
    Other(String),
}

impl fmt::Display for PostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PostError::PostNotFoundError(s) => write!(f, "Post Not Found Error: {}", s),
            PostError::Other(s) => write!(f, "Other Error!: {}", s),
            PostError::PostDBError(s) => write!(f, "Database Error: {}", s),
        }
    }
}
