use thiserror::Error;

// user-facing error messages
// more verbose info is logged at the error!() level
#[derive(Error, Debug)]
pub enum ImagerError {
    #[error("Failed to unmount volume. Please eject the device, re-insert, and try again.")]
    VolumeUnmount,
    #[error("Failed to acquire exclusive lock for device. Please eject the device, re-insert, and try again.")]
    VolumeLock,
    #[error("Failed to format disk. Please eject the device, re-insert, and try again.")]
    FormatDisk,
}