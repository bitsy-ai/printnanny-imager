use anyhow::{Context, Result};
use human_bytes::human_bytes;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::io::{self, prelude::*, BufReader};
use std::process::Command;
use std::process::Stdio;
use std::string::String;

use super::app;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ProgressPayload {
    msg: String,
}

#[cfg(target_os = "macos")]
pub fn empty_darwin_disk_list() -> Vec<DarwinDisk> {
    return vec![];
}

#[cfg(target_os = "macos")]
pub fn create_darwin_authorization(
    filename: &str,
) -> Result<security_framework::authorization::Authorization> {
    let rights = security_framework::authorization::AuthorizationItemSetBuilder::new()
        .add_right(format!("sys.openfile.readwrite.{}", filename))?
        .build();

    Ok(security_framework::authorization::Authorization::new(
        Some(rights),
        None,
        security_framework::authorization::Flags::INTERACTION_ALLOWED
            | security_framework::authorization::Flags::EXTEND_RIGHTS
            | security_framework::authorization::Flags::PREAUTHORIZE,
    )?)
}

#[cfg(target_os = "macos")]
pub fn authext_darwin(disk: &str) -> Result<security_framework::authorization::Authorization> {
    // create authorization object
    let auth = create_darwin_authorization(disk)?;
    Ok(auth)
    // let form = auth.make_external_form()?;
    // let inbytes =
    //     unsafe { slice::from_raw_parts(form.bytes.as_ptr() as *const u8, form.bytes.len()) };
    // Ok(inbytes.to_vec())
}

#[cfg(target_os = "macos")]
pub async fn write_image_darwin(image_path: String, disk: String) -> Result<()> {
    info!("Attempting to unmount disk {}", &disk);
    // unmount disk
    let unmount_output = Command::new("/usr/sbin/diskutil")
        .args(&["unmountDisk", &disk])
        .output()?;
    match unmount_output.status.success() {
        true => {
            info!("Successfully unmounted disk {}", &disk);
        }
        false => {
            error!(
                "Error unmounting disk {}: {}",
                &disk,
                String::from_utf8(unmount_output.stderr)?
            );
        }
    };

    info!("Writing {} to {}", &image_path, &disk);
    let auth = authext_darwin(&disk)?;

    let dd_handle = auth.execute_with_privileges_piped(
        "/bin/dd",
        &[
            format!("if={}", &image_path),
            format!("of={}", &disk),
            "bs=4M".to_string(),
            "status=progress".to_string(),
        ],
        security_framework::authorization::Flags::DEFAULTS,
    )?;
    info!("Created handle for /bin/dd {:?}", dd_handle);

    let reader = BufReader::new(dd_handle);
    for line in reader.lines() {
        let line = line?;
        let payload = ProgressPayload {
            msg: line.to_string(),
        };
        app::TauriApp::emit("write_image_progress", payload);
        info!("{}", &line);
    }

    info!("Finished reading lines from fd");

    Ok(())
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
        .output()
        .context("Command failed: diskutil list -plist physical")?;
    // convert plist to json
    // let disks_list_plist_stdio = String::from_utf8(disks_list_plist_child.stdout)?;
    let mut disks_list_json_child = Command::new("plutil")
        .args(&["-convert", "json", "-r", "-o", "-", "-"])
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
            .args(&["info", "-plist", disk])
            .stdout(Stdio::piped())
            .output()
            .context(format!("Command failed: diskutil info -plist {}", disk))?;
        // let disks_info_plist_stdio = disks_info_plist_child.stdout;

        // convert plist to json
        let mut disks_info_json_child = Command::new("plutil")
            .args(&["-convert", "json", "-r", "-o", "-", "-"])
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
