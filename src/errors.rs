use std::io::Error as StdError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to set sepolicy, Err: {source}")]
    SetSepolicyFailed {
        #[source]
        source: StdError,
    },
    #[error("Failed to nuke {path}, Err: {source}")]
    NukeFailed {
        path: String,
        #[source]
        source: StdError,
    },
    #[error("Failed to get safe mode status, Err: {source}")]
    GetSafeModeFailed {
        #[source]
        source: StdError,
    },
    #[error("feature_id is not set")]
    MissingFeatureId,
    #[error("value is not set")]
    MissingFeatureValue,
    #[error("Failed to get feature_id {id}, Err: {source}")]
    GetFeatureFailed {
        id: u32,
        #[source]
        source: StdError,
    },
    #[error("Failed to set feature_id {id} to {value}, Err: {source}")]
    SetFeatureFailed {
        id: u32,
        value: u64,
        #[source]
        source: StdError,
    },
    #[error("Failed to add try-umount list {path}, Err: {source}")]
    TryUmountAddFailed {
        path: String,
        #[source]
        source: StdError,
    },
    #[error("Failed to wipe try-umount list, Err: {source}")]
    TryUmountWipeFailed {
        #[source]
        source: StdError,
    },
}
