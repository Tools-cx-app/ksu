use std::io::Error as StdError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to set dynamic manager, Err: {source}")]
    Set {
        #[source]
        source: StdError,
    },
    #[error("Failed to get dynamic manager, Err: {source}")]
    Get {
        #[source]
        source: StdError,
    },
    #[error("Failed to clear dynamic manager, Err: {source}")]
    Clear {
        #[source]
        source: StdError,
    },
    #[error("size is not set")]
    MissingSize,
    #[error("hash is not set")]
    MissingHash,
}
