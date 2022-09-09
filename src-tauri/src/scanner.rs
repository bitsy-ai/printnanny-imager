use serde::{Deserialize, Serialize};

#[cfg(windows)]
#[derive(Serialize, Deserialize, Debug)]
pub struct WindowsDiskDrive {
    #[serde(rename = "BytesPerSector")]
    pub bytes_per_sector: i32,
    #[serde(rename = "Capabilities")]
    pub capabilities: Vec<i32>,
    #[serde(rename = "CapabilityDescriptions")]
    pub capability_descriptions: Vec<String>,
    #[serde(rename = "Caption")]
    pub caption: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "DeviceID")]
    pub device_id: String,
    #[serde(rename = "FirmwareRevision")]
    pub firmware_revision: String,
    #[serde(name = "Manufacturer")]
    pub manufacturer: String,
    #[serde(name = "MediaType")]
    pub media_type: String,
    #[serde(name = "Model")]
    pub model: String,
    #[serde(name = "Partitions")]
    pub partitions: i32,
    #[serde(name = "PNDeviceID")]
    pub pn_device_id: String,
    #[serde(name = "SerialNumber")]
    pub serial_number: String,
    #[serde(name = "Size")]
    pub size: u64,
    #[serde(name = "Status")]
    pub status: String,
}

#[cfg(windows)]
pub fn scan_windows_drives() {
    let query = "GET-WMIOBJECT -query 'SELECT * from win32_diskdrive' | ConvertTo-Json";
}
