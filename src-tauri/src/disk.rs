use async_process::Command;
use std::process::Stdio;

use anyhow::{Context, Result};
use human_bytes::human_bytes;
use log::{info, warn};
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SerializableDisk {
//     pub type_: String,
//     pub name: String,
//     #[serde(rename = "fileSystem")]
//     pub file_system: String,
//     #[serde(rename = "mountPoint")]
//     pub mount_point: String,
//     #[serde(rename = "totalSpace")]
//     pub total_space: u64,
//     #[serde(rename = "totalSpacePretty")]
//     pub total_space_pretty: String,
//     #[serde(rename = "availableSpace")]
//     pub available_space: u64,
//     #[serde(rename = "availableSpacePretty")]
//     pub available_space_pretty: String,
//     #[serde(rename = "isRemoveable")]
//     pub is_removable: bool,
// }

// pub fn list_serializable_disks() -> Vec<SerializableDisk> {
//     let mut sys = System::new();
//     sys.refresh_disks_list();
//     sys.disks()
//         .iter()
//         .map(|d| SerializableDisk {
//             type_: format!("{:?}", d.type_()),
//             name: d.name().to_string_lossy().to_string(),
//             file_system: String::from_utf8_lossy(d.file_system()).to_string(),
//             mount_point: d.mount_point().display().to_string(),
//             total_space: d.total_space(),
//             total_space_pretty: human_bytes(d.total_space() as f64),
//             available_space: d.available_space(),
//             available_space_pretty: human_bytes(d.available_space() as f64),
//             is_removable: d.is_removable(),
//         })
//         .collect::<Vec<SerializableDisk>>()
// }

#[cfg(target_os = "macos")]
pub fn empty_darwin_disk_list() -> Vec<DarwinDisk> {
    return vec![];
}

// DarwinDisk is deserialized from:
// $ diskutil info -plist <disk> | plutil -convert json -r -o - -
#[cfg(target_os = "macos")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DarwinDisk {
    #[serde(rename(deserialize = "Bootable"))]
    pub bootable: bool,
    #[serde(rename(deserialize = "BusProtocol"))]
    pub bus_protocol: String,
    #[serde(rename(deserialize = "IORegistryEntryName"))]
    pub display_name: String,
    #[serde(rename(deserialize = "DeviceIdentifier"))]
    pub device_id: String,
    #[serde(rename(deserialize = "DeviceNode"))]
    pub path: String,
    #[serde(rename(deserialize = "Size"))]
    pub size: u64,
    #[serde(rename(deserialize = "Removable"))]
    pub is_removable: bool,
    #[serde(rename(deserialize = "ParentWholeDisk"))]
    pub parent_whole_disk: String,
    #[serde(default = "empty_darwin_disk_list")]
    pub partitions: Vec<Self>,
    #[serde(rename(deserialize = "VolumeName"))]
    pub volume_name: String,
}

impl DarwinDisk {
    pub fn is_whole_disk(&self) -> bool {
        self.parent_whole_disk == self.device_id
    }
    pub fn is_partition(&self) -> bool {
        self.parent_whole_disk != self.device_id
    }
}

// DarwinDiskList is deserialized from:
// $ diskutil list -plist physical
#[cfg(target_os = "macos")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DarwinDiskList {
    #[serde(rename(deserialize = "AllDisks"))]
    pub all_disks: Vec<String>,
}

#[cfg(target_os = "macos")]
pub fn merge_darwin_disks(disks: Vec<DarwinDisk>) -> Vec<DarwinDisk> {
    // seek "parent" whole disk nodes
    let whole_disks = disks.iter().cloned().filter(|d| d.is_whole_disk());

    // seek partition disk nodes
    let partitions: Vec<DarwinDisk> = disks.iter().cloned().filter(|d| d.is_partition()).collect();

    // assign partitions to parent disk
    whole_disks
        .map(|mut d| {
            d.partitions = partitions
                .iter()
                .filter(|p| p.parent_whole_disk == d.device_id)
                .cloned()
                .collect();
            d
        })
        .collect()
}

#[cfg(target_os = "macos")]
pub async fn list_removeable_disks_darwin() -> Result<Vec<DarwinDisk>> {
    // get a list of physical disks
    // equivalent to:
    // $ diskutil list -plist physical | plutil -convert json -r -o - -
    let disks_list_plist_child = Command::new("diskutil")
        .args(&["list", "-plist", "physical"])
        .stdout(Stdio::piped())
        .spawn()
        .context("Command failed: diskutil list -plist physical")?;
    // convert plist to json
    let disks_list_plist_stdio = disks_list_plist_child.stdout.unwrap().into_stdio().await?;
    let disks_list_json_child = Command::new("plutil")
        .args(&["-convert", "json", "-r", "-o", "-", "-"])
        .stdin(disks_list_plist_stdio) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()
        .context("Command failed: plutil -convert json -r -o - -")?;

    // parse json
    let disks_list_json = disks_list_json_child.output().await?;
    let disks_list: DarwinDiskList = serde_json::from_slice(&disks_list_json.stdout)?;

    // for each disk, get detailed info. Equivalent to:
    // $ diskutil info -plist <disk> | plutil -convert json -r -o - -
    let mut result: Vec<DarwinDisk> = vec![];
    for disk in disks_list.all_disks.iter() {
        // get detailed information about <disk>
        let disks_info_plist_child = Command::new("diskutil")
            .args(&["info", "-plist", disk])
            .stdout(Stdio::piped())
            .spawn()
            .context(format!("Command failed: diskutil info -plist {}", disk))?;
        let disks_info_plist_stdio = disks_info_plist_child.stdout.unwrap().into_stdio().await?;

        // convert plist to json
        let disks_info_json_child = Command::new("plutil")
            .args(&["-convert", "json", "-r", "-o", "-", "-"])
            .stdin(disks_info_plist_stdio) // Pipe through.
            .stdout(Stdio::piped())
            .spawn()
            .context("Command failed: plutil -convert json -r -o - -")?;
        // parse json
        let disks_info_json = disks_info_json_child.output().await?;
        let darwin_disk: DarwinDisk = serde_json::from_slice(&disks_info_json.stdout)?;
        if darwin_disk.is_removable {
            info!("Detected removeable drive {:?}", &disks_info_json);
            result.push(darwin_disk);
        } else {
            warn!("Skipping non-removeable disk {}", disk);
        }
    }
    result = merge_darwin_disks(result);
    Ok(result)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CrossPlatformDisk {
    pub bootable: bool,
    #[serde(rename(serialize = "busProtocol"))]
    pub bus_protocol: String,
    #[serde(rename(serialize = "displayName"))]
    pub display_name: String,
    #[serde(rename(serialize = "deviceId"))]
    pub device_id: String,
    pub path: String,
    pub size: u64,
    #[serde(rename(serialize = "sizePretty"))]
    pub size_pretty: String,
    #[serde(rename(serialize = "isRemoveable"))]
    pub is_removable: bool,
    pub partitions: Vec<Self>,
    #[serde(rename(serialize = "volumeName"))]
    pub volume_name: String,
}

impl From<&DarwinDisk> for CrossPlatformDisk {
    fn from(disk: &DarwinDisk) -> Self {
        Self {
            bootable: disk.bootable.clone(),
            bus_protocol: disk.bus_protocol.clone(),
            display_name: disk.display_name.clone(),
            device_id: disk.device_id.clone(),
            path: disk.path.clone(),
            size: disk.size,
            size_pretty: human_bytes(disk.size as f64),
            is_removable: disk.is_removable,
            volume_name: disk.volume_name.clone(),
            partitions: disk.partitions.iter().map(|d| Self::from(d)).collect(),
        }
    }
}

pub async fn list_disks() -> Result<Vec<CrossPlatformDisk>> {
    if cfg!(target_os = "macos") {
        let result = list_removeable_disks_darwin()
            .await?
            .iter()
            .map(|d| CrossPlatformDisk::from(d))
            .collect();
        return Ok(result);
    }
    Ok(vec![])
}
