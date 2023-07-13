//! Minimalized (if that's a word) custom error handling bullshit for this small ass script

use std::sync::{MutexGuard, PoisonError};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Error {
    IO(std::io::Error), //  sure
    Sync,               //  don't care about this
    JoinThread,         //  don't care either
}

impl From<std::io::Error> for Error {
    fn from(pair: std::io::Error) -> Self {
        Self::IO(pair)
    }
}

impl From<PoisonError<MutexGuard<'_, u16>>> for Error {
    fn from(_: PoisonError<MutexGuard<'_, u16>>) -> Self {
        Self::Sync
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::IO(e) => e.to_string(),
            Self::Sync => "A unexpected poison error has occurred".to_string(),
            Self::JoinThread => "Unexpected error while waiting for thread to finish".to_string(),
        })
    }
}

//  custom [`Debug`] for clean exit error
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}
