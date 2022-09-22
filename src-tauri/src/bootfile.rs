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
    // let diskutil_info_plist = Command::new("plutil")
    //     .args(&["-convert", "json", "-r", "-o", "-", "-"])
    //     .stdin(diskutil_info_child.stdout.unwrap()) // Pipe through.
    //     .stdout(Stdio::piped())
    //     .spawn()?;
    // // parse json
    // let diskutil_json = diskutil_info_plist.wait_with_output()?;
    // let mount = serde_json::from_slice(&diskutil_json.stdout)?;
    // info!("get_boot_mountpoint: {:?}", &mount);
    // Ok(mount)
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
    unimplemented!("_write_bootfile is not yet implemented on windows")
}

#[tauri::command(async)]
pub fn write_bootfile(disk_path: String, filename: String, contents: String) -> Result<(), String> {
    match _write_bootfile(disk_path, filename, contents) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}
