
use std::process::Command;
use std::string::String;

use anyhow::{Result};
use human_bytes::human_bytes;
use log::{ info};
use serde::{Deserialize, Serialize};

#[cfg(unix)]
use {
    anyhow::Context,
    std::fs::File,
    std::io::IoSliceMut,
    std::io::Write,
    std::time::Instant,
    std::io::{prelude::*, BufReader},
    std::process::Stdio
};

// DarwinDiskList is deserialized from:
// $ diskutil list -plist physical
#[cfg(target_os = "macos")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DarwinDiskList {
    #[serde(rename(deserialize = "AllDisks"))]
    pub all_disks: Vec<String>,
}

#[cfg(target_os = "macos")]
pub fn empty_darwin_disk_list() -> Vec<DarwinDisk> {
    vec![]
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

#[cfg(target_os = "macos")]
impl DarwinDisk {
    pub fn is_whole_disk(&self) -> bool {
        self.parent_whole_disk == self.device_id
    }
    pub fn is_partition(&self) -> bool {
        self.parent_whole_disk != self.device_id
    }
}

// WindowsDisk is serialized from:
// GET-WMIOBJECT -query 'SELECT * from win32_diskdrive WHERE MediaType="Removable Media"' | ConvertTo-Json
#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowsDisk {
    #[serde(rename(deserialize = "DeviceID"))]
    pub device_id: String,
    #[serde(rename(deserialize = "Caption"))]
    pub display_name: String,
    #[serde(rename(deserialize = "MediaType"))]
    pub media_type: String,
    #[serde(rename(deserialize = "InterfaceType"))]
    pub interface_type: String,
    #[serde(rename(deserialize = "Name"))]
    pub volume_name: String,
    #[serde(rename(deserialize = "SerialNumber"))]
    pub serial_number: String,
    #[serde(rename(deserialize = "Size"))]
    pub size: u64,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[cfg(target_os = "macos")]
impl From<&DarwinDisk> for CrossPlatformDisk {
    fn from(disk: &DarwinDisk) -> Self {
        Self {
            bootable: disk.bootable,
            bus_protocol: disk.bus_protocol.clone(),
            display_name: disk.display_name.clone(),
            device_id: disk.device_id.clone(),
            path: disk.path.clone(),
            size: disk.size,
            size_pretty: human_bytes(disk.size as f64),
            is_removable: disk.is_removable,
            volume_name: disk.volume_name.clone(),
            partitions: disk.partitions.iter().map(Self::from).collect(),
        }
    }
}

#[cfg(target_os = "windows")]
impl From<&WindowsDisk> for CrossPlatformDisk {
    fn from(disk: &WindowsDisk) -> Self {
        Self {
            bootable: true, // TODO: how to read bootable bit?
            bus_protocol: disk.interface_type.clone(),
            display_name: disk.display_name.clone(),
            device_id: disk.device_id.clone(),
            path: disk.volume_name.clone(),
            size: disk.size,
            size_pretty: human_bytes(disk.size as f64),
            is_removable: disk.media_type == "Removable Media", // 7 -> "Supports Removable Media" capability
            volume_name: disk.volume_name.clone(),
            partitions: vec![]
        }
    }
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
        .args(["list", "-plist", "physical"])
        .stdout(Stdio::piped())
        .output()
        .context("Command failed: diskutil list -plist physical")?;
    // convert plist to json
    // let disks_list_plist_stdio = String::from_utf8(disks_list_plist_child.stdout)?;
    let mut disks_list_json_child = Command::new("plutil")
        .args(["-convert", "json", "-r", "-o", "-", "-"])
        .stdin(Stdio::piped()) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()
        .context("Command failed: plutil -convert json -r -o - -")?;
    // write to plutil stdin
    let mut plutil_stdin = disks_list_json_child
        .stdin
        .take()
        .expect("Failed to open stdin");
    std::thread::spawn(move || {
        plutil_stdin
            .write_all(&disks_list_plist_child.stdout)
            .expect("Failed to write to plutil stdin");
    });

    // parse json
    let disks_list_json = disks_list_json_child.wait_with_output()?;
    let disks_list: DarwinDiskList = serde_json::from_slice(&disks_list_json.stdout)?;

    // for each disk, get detailed info. Equivalent to:
    // $ diskutil info -plist <disk> | plutil -convert json -r -o - -
    let mut result: Vec<DarwinDisk> = vec![];
    for disk in disks_list.all_disks.iter() {
        // get detailed information about <disk>
        let disks_info_plist_child = Command::new("diskutil")
            .args(["info", "-plist", disk])
            .stdout(Stdio::piped())
            .output()
            .context(format!("Command failed: diskutil info -plist {}", disk))?;

        // convert plist to json
        let mut disks_info_json_child = Command::new("plutil")
            .args(["-convert", "json", "-r", "-o", "-", "-"])
            .stdin(Stdio::piped()) // Pipe through.
            .stdout(Stdio::piped())
            .spawn()
            .context("Command failed: plutil -convert json -r -o - -")?;
        // write to plutil stdin
        let mut plutil_stdin = disks_info_json_child
            .stdin
            .take()
            .expect("Failed to open stdin");
        std::thread::spawn(move || {
            plutil_stdin
                .write_all(&disks_info_plist_child.stdout)
                .expect("Failed to write to plutil stdin");
        });

        // parse json
        let disks_info_json = disks_info_json_child.wait_with_output()?;
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

#[cfg(target_os = "windows")]
pub fn list_removeable_disks_windows() -> Result<Vec<WindowsDisk>> {
    let cmd = Command::new("powershell.exe")
        .args(["GET-WMIOBJECT", "-query", "'SELECT * from win32_diskdrive WHERE MediaType!=null'", "|", "ConvertTo-Json"]).output()?;
    
    let windows_disks: Vec<WindowsDisk> = serde_json::from_str(&String::from_utf8_lossy(&cmd.stdout))?;
    info!("Got disks {:?}", &windows_disks);
    Ok(windows_disks.iter().cloned().filter(|d| d.media_type == "Removable Media").collect())
}


#[cfg(target_os = "macos")]
pub async fn list_disks() -> Result<Vec<CrossPlatformDisk>> {
    let result = list_removeable_disks_darwin()
        .await?
        .iter()
        .map(CrossPlatformDisk::from)
        .collect();
    return Ok(result);
}

#[cfg(target_os = "linux")]
pub async fn list_disks() -> Result<Vec<CrossPlatformDisk>> {
    // let result =  list_removeble_disks_
    unimplemented!("list_disks is not implemented for target_os=linux")
}

#[cfg(target_os = "windows")]
pub async fn list_disks() -> Result<Vec<CrossPlatformDisk>> {
    let result = list_removeable_disks_windows()?;
    Ok(result.iter().map(CrossPlatformDisk::from).collect())
}

