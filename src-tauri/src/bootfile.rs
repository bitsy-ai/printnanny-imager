use std::fmt::format;

use crate::error::ImagerError;
#[cfg(unix)]
use {
    log::{error, info},
    serde::{Deserialize, Serialize},
    std::fs,
    std::process::Command,
    std::process::Stdio,
};

#[cfg(target_os = "windows")]
use {
    log::{error, info},
    std::process::Command,
    serde::{Deserialize, Serialize},
    std::process::Stdio,
    std::path::PathBuf,
    super::disk::get_windows_drivenum,

};

// DarwinDisk is deserialized from:
// $ diskutil info -plist <disk><part> | plutil -convert json -r -o - -
#[cfg(target_os = "macos")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DarwinMount {
    #[serde(rename(deserialize = "DeviceIdentifier"))]
    pub device_id: String,
    #[serde(rename(deserialize = "DeviceNode"))]
    pub device_node: String,
    #[serde(rename(deserialize = "MountPoint"))]
    pub mountpoint: String,
    #[serde(rename(deserialize = "Writable"))]
    pub writable: bool,
}

// WindowsDriveToDiskPartition is serialized from:
// GET-WMIOBJECT -query 'ASSOCIATORS OF {Win32_DiskDrive.DeviceID="<id>"} WHERE AssocClass = Win32_DiskDriveToDiskPartition' | ConvertTo-Json
#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowsDriveToDiskPartition {
    #[serde(rename(deserialize = "DeviceID"))]
    pub device_id: String,
}

// WindowsLogicalDisk serialized from:
// example device_id: "Disk #3, Partition #0"
// GET-WMIOBJECT -query 'ASSOCIATORS OF {Win32_DiskPartition.DeviceID="Disk #3, Partition #0"} where AssocClass = Win32_LogicalDiskToPartition' | ConvertTo-Json
#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowsLogicalDisk {
    #[serde(rename(deserialize = "DeviceID"))]
    pub device_id: String,
}

// DarwinDisk is deserialized from a series of WMIOBJECT queries
// Get physical disks:
// $ GET-WMIOBJECT -query 'select * from Win32_diskdrive' | ConvertTo-Json
// Get partitions for physical disk:
// $ GET-WMIOBJECT -query 'ASSOCIATORS OF {Win32_DiskDrive.DeviceID="<id>"} WHERE AssocClass = Win32_DiskDriveToDiskPartition' | ConvertTo-Json
// Get logical volume mount for partition:
// $ GET-WMIOBJECT -query 'ASSOCIATORS OF {Win32_DiskPartition.DeviceID="Disk #3, Partition #0"} where AssocClass = Win32_LogicalDiskToPartition' | ConvertTo-Json
#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowsMount {
    #[serde(rename(deserialize = "DeviceID"))]
    pub device_id: String,
    #[serde(rename(deserialize = "VolumeName"))]
    pub volume_name: String,
    #[serde(rename(deserialize = "VolumeDirty"))]
    pub volume_dirty: bool,
}

#[cfg(target_os = "windows")]
fn get_boot_mountpoint(disk_path: &str) -> Result<WindowsMount, ImagerError> {

    let drivenum = get_windows_drivenum(disk_path);
    let query_filter = format!("Disk #{}, Partition #0", &drivenum);
    let query =  format!("'ASSOCIATORS OF {{Win32_DiskPartition.DeviceID=\"{}\"}} where AssocClass = Win32_LogicalDiskToPartition'", &query_filter);
    // scan until mountpoint query returns non-empty
    let mut attempts = 20;
    while attempts > 0 {
        let cmd = Command::new("powershell.exe")
        .args([
            "GET-WMIOBJECT",
            "-query",
            &query,
            "|",
            "ConvertTo-Json",
        ])
        .output()?;
        
        let json_result = &String::from_utf8_lossy(&cmd.stdout);
        let windows_mount = serde_json::from_str(&json_result);
        match windows_mount {
            Ok(m) => {
                return Ok(m)
            },
            Err(e) => {
                attempts -= 1;
                info!(
                    "get_boot_mountpoint failed with error {:?} - {} attempts remaining",
                    &e, attempts
                );
            }
        }
    }
    Err(ImagerError::BootfileWrite {
        path: disk_path.to_string(),
    })
}

#[cfg(target_os = "macos")]
fn get_boot_mountpoint(disk_path: &str) -> Result<DarwinMount, ImagerError> {
    let dnode = format!("{}s1", disk_path);

    // scan for disk until mountpoint is non-empty
    let mut attempts = 20;
    while attempts > 0 {
        let diskutil_info_child = Command::new("diskutil")
            .args(&["info", "-plist", &dnode])
            .stdout(Stdio::piped())
            .spawn()?;
        let diskutil_info_plist = Command::new("plutil")
            .args(&["-convert", "json", "-r", "-o", "-", "-"])
            .stdin(diskutil_info_child.stdout.unwrap()) // Pipe through.
            .stdout(Stdio::piped())
            .spawn()?;
        // parse json
        let diskutil_json = diskutil_info_plist.wait_with_output()?;
        let mount: DarwinMount = serde_json::from_slice(&diskutil_json.stdout)?;
        if mount.mountpoint == "" {
            attempts -= 1;
            info!(
                "get_boot_mountpoint returned empty mountpoint {:?} - {} attempts remaining",
                &mount, attempts
            );
        } else {
            info!("get_boot_mountpoint: {:?}", &mount);
            return Ok(mount);
        }
    }
    Err(ImagerError::BootfileWrite {
        path: disk_path.to_string(),
    })
}

#[cfg(target_os = "macos")]
fn _write_bootfile(
    disk_path: String,
    filename: String,
    contents: String,
) -> Result<(), ImagerError> {
    // get mountpoint for first partition of physical disk_path

    use std::path::PathBuf;
    let mount = get_boot_mountpoint(&disk_path)?;
    let outpath = PathBuf::from(&mount.mountpoint).join(filename);

    // bail if mountpoint is not mounted or writable
    match mount.writable {
        true => {
            fs::write(&outpath, contents)?;
            info!("Success! Wrote {}", outpath.display());
            Ok(())
        }
        false => {
            let pathstr = outpath.display().to_string();
            error!(
                "Failed to write boot file {} to volume mount {:?}",
                &pathstr, &mount
            );
            Err(ImagerError::BootfileWrite {
                path: outpath.display().to_string(),
            })
        }
    }?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn _write_bootfile(
    disk_path: String,
    filename: String,
    contents: String,
) -> Result<(), ImagerError> {
    unimplemented!("_write_bootfile is not yet implemented on linux")
}

#[cfg(target_os = "windows")]
fn _write_bootfile(
    disk_path: String,
    filename: String,
    contents: String,
) -> Result<(), ImagerError> {
    let mountpoint = get_boot_mountpoint(&disk_path)?;
    let volume = PathBuf::from(&format!("{}\\", mountpoint.device_id)).join(filename);
    match std::fs::write(&volume, contents){
        Ok(()) => {
            info!("Success! Wrote {:?}", &volume);
            Ok(())
        },
        Err(e) => Err(ImagerError::BootfileWrite { path: volume.display().to_string() })
    }
}

#[tauri::command(async)]
pub fn write_bootfile(disk_path: String, filename: String, contents: String) -> Result<(), String> {
    match _write_bootfile(disk_path, filename, contents) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}
