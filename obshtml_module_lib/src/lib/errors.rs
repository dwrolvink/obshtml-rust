use thiserror; 

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error.")]
    Internal(String),

    #[error("Not found.")]
    NotFound,

    #[error("Run failed. Reason: {0}")]
    RunFailed(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Wrong number of arguments")]
    WrongNumberOfArguments(String),
    
    #[error("Use of wrong path type: {0}")]
    WrongPathType(String),
}