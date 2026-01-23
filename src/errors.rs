use std::io::Error as StdError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FeatureError {
    #[error("feature_id is not set")]
    MissingFeatureId,
    #[error("value is not set")]
    MissingValue,
    #[error("Failed to get feature_id {id}, Err: {source}")]
    GetFailed {
        id: u32,
        #[source]
        source: StdError,
    },
    #[error("Failed to set feature_id {id} to {value}, Err: {source}")]
    SetFailed {
        id: u32,
        value: u64,
        #[source]
        source: StdError,
    },
}

#[derive(Error, Debug)]
pub enum NukeError {
    #[error("Failed to nuke {path}, Err: {source}")]
    NukeFailed {
        path: String,
        #[source]
        source: StdError,
    },
}

#[derive(Error, Debug)]
pub enum TryUmountError {
    #[error("Failed to add try-umount list {path}, Err: {source}")]
    UmountFailed {
        path: String,
        #[source]
        source: StdError,
    },
    #[error("Failed to wipe try-umount list, Err: {source}")]
    WipeFailed {
        #[source]
        source: StdError,
    },
}
