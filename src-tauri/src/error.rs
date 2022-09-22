use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

// user-facing error messages
// more verbose info is logged at the error!() level
#[derive(Error, Debug)]
pub enum ImagerError {
    #[error(
        "Failed to write boot file {path}. Please eject the device, re-insert, and try again."
    )]
    BootfileWrite { path: String },
    #[error("Failed to unmount volume. Please eject the device, re-insert, and try again.")]
    VolumeUnmount,
    #[error("Failed to acquire exclusive lock for device. Please eject the device, re-insert, and try again.")]
    VolumeLock,
    #[error("Failed to format disk. Please eject the device, re-insert, and try again.")]
    FormatDisk,
    #[error("Failed to open disk {path}. Please eject the device, re-insert, and try again.")]
    OpenDisk { path: String },
    #[error(transparent)]
    Io {
        #[from]
        source: io::Error,
    },
    #[error(transparent)]
    SerdeJson {
        #[from]
        source: serde_json::Error,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}
