use std::io;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("getting process info failed: {0}")]
    ProcessInfo(#[from] ProcessInfoError),

    #[error("failed getting command binary (got \"{0}\")")]
    NoBinary(String),

    #[error("no shell found")]
    NoShellFound,

    #[error("failed getting pid: {0}")]
    FailedGettingPid(&'static str),
}

/// Contains information about a failed process info collection.
#[derive(thiserror::Error, Debug)]
pub enum ProcessInfoError {
    #[error("command error: {0}")]
    CommandFailed(#[from] io::Error),

    #[error("unexpected EOF")]
    UnexpectedEof,

    #[error("unexpected output: {msg} (got \"{output}\")")]
    UnexpectedOutput { msg: &'static str, output: String },
}
