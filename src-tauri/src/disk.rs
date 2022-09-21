use std::io::{prelude::*, BufReader};
use std::process::Command;
use std::process::Stdio;
use std::string::String;
use std::time::Instant;
use std::fs::File;
use std::{io::BufWriter};
use anyhow::{anyhow, Context, Result};
use std::path::Path;
use human_bytes::human_bytes;
use log::{error, info, warn, debug};
use serde::{Deserialize, Serialize};

use super::error::ImagerError;

#[cfg(target_os = "windows")]
use {
    windows::{core::*, Win32::Storage::FileSystem, Win32::Foundation, Win32::System::IO, Win32::System::Ioctl},
    regex::Regex,
    std::os::windows::io::FromRawHandle
};

use super::app;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WriteImageProgress {
    // pub percent_complete: i32,
    pub bytes_written: u64,
    pub bytes_total: u64,
    pub elapsed: u64,
    pub label: String,
    // pub write_speed: String,
    // pub stdout_line: String,
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
}

#[cfg(target_os = "macos")]
pub fn unmount_disk_darwin(disk: &str, bytes_total: u64) -> Result<()> {
    let payload = WriteImageProgress {
        bytes_written: 0_u64,
        bytes_total,
        label: "Unmounting disk".to_string(),
        elapsed: 0_u64,
    };
    app::TauriApp::emit(app::EVENT_IMAGE_WRITE_PROGRESS, payload);
    info!("Attempting to unmount disk {}", &disk);
    // unmount disk
    let unmount_output = Command::new("/usr/sbin/diskutil")
        .args(["unmountDisk", &disk])
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
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn write_image(image_path: String, disk: String, device_id: String) -> Result<()> {
    unimplemented!("write_image is not implemented for target_os=linux")
}

#[cfg(target_os = "windows")]
pub fn partition_disk_windows(drivenum: &str)-> Result<()> {
    // write diskpart script to temp_dir
    // create partition primary
    // select partition 1
    // format fs=fat32 quick
    // active
    // assign
    let mut f = tempfile::NamedTempFile::new()?;
    let diskpart_script = format!("select disk {}
    clean
    rescan
    ", drivenum);
    info!("Running diskpart script {}", &diskpart_script);
    f.write_all(&diskpart_script.as_bytes())?;

    let child = Command::new("diskpart.exe")
        .args(
            ["/s", &f.path().display().to_string()]
        ).spawn()?;
    info!("Running diskpart cmd {:?}", &child);
    let output = child.wait_with_output()?;
    match output.status.success() {
        true => {
            info!("Success! Cleaned {} - drive is now RAW and writeable with Windows direct access policies", &drivenum);
            Ok(())
        },
        false => {
            error!("Error running disk format {:?}",&output);
            Err(ImagerError::FormatDisk)
        }
    }?;
    Ok(())
}

// WindowsDriveToDiskPartition is serialized from:
// GET-WMIOBJECT -query 'ASSOCIATORS OF {Win32_DiskDrive.DeviceID="<id>" } WHERE AssocClass = Win32_DiskDriveToDiskPartition' | ConvertTo-Json
#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowsDriveToDiskPartition {
    #[serde(rename(deserialize = "DeviceID"))]
    pub device_id: String,
}

// WindowsLogicalDisk serialized from:
// example device_id: "3"
// GET-WMIOBJECT -query 'ASSOCIATORS OF {Win32_DiskPartition.DeviceID="<device_id>" } WHERE AssocClass = Win32_DiskDriveToDiskPartition' | ConvertTo-Json
#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowsLogicalDisk {
    #[serde(rename(deserialize = "DeviceID"))]
    pub device_id: String,
}

// Reverse query for letter associated with logical disk partition
// example device_id: "Disk #3, Partition #0"
// ref: https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--disks-and-file-systems
#[cfg(target_os = "windows")]
fn get_drive_letter_windows(device_id: &str) -> Result<String> {
    // query associated data model
    let query = format!("'ASSOCIATORS OF {{Win32_DiskDrive.DeviceID={device_id:?}}} WHERE AssocClass = Win32_DiskDriveToDiskPartition'", device_id=device_id);
    let output = Command::new("powershell.exe")
        .args(["GET-WMIOBJECT", "-query", &query, "|", "ConvertTo-Json"]).output()?;

    let assoc_result_utf8 = String::from_utf8_lossy(&output.stdout).to_string();
    debug!("Query {} output {:?}", &query, &assoc_result_utf8);

    let drive_to_disk_partition: WindowsDriveToDiskPartition = serde_json::from_str(&assoc_result_utf8).context(format!("Failed to deserialize Win32_DiskDriveToDiskPartition output: {}", &assoc_result_utf8))?;

    let query = format!("'ASSOCIATORS OF {{Win32_DiskPartition.DeviceId=\"{device_id}\"}} WHERE AssocClass = Win32_LogicalDiskToPartition'", device_id=drive_to_disk_partition.device_id);
    let output = Command::new("powershell.exe")
        .args(["GET-WMIOBJECT", "-query", &query, "|", "ConvertTo-Json"]).output()?;
    
    

    let assoc_result_utf8 = String::from_utf8_lossy(&output.stdout).to_string();
    debug!("Query {} output {:?}", &query, &assoc_result_utf8);

    let logical_disk: WindowsLogicalDisk = serde_json::from_str(&assoc_result_utf8).context(format!("Failed to deserialize Win32_LogicalDiskToPartition output: {}", &assoc_result_utf8))?;
    // WindowsLogicalDisk.device_id will be the mounted drive letter, e.g. "D:"
    Ok(logical_disk.device_id)
}

#[cfg(target_os = "windows")]
pub fn lock_volume(handle: Foundation::HANDLE, ) -> Result<(), ImagerError>{
    info!("Enabling FSCTL_ALLOW_EXTENDED_DASD_IO via DeviceIoControl API");
    unsafe { IO::DeviceIoControl(
        handle, 
        Ioctl::FSCTL_ALLOW_EXTENDED_DASD_IO,
         None,
        None,
        None, 
        None, ) };

    let mut attempts = 20;
    let mut success: bool = false;
    while attempts > 0 {
        let result = unsafe { IO::DeviceIoControl(
            handle, 
            Ioctl::FSCTL_LOCK_VOLUME,
             None,
            None,
            None, 
            None, ) };
        match result.as_bool() {
            true => { 
                info!("Success! Locked volume with DeviceIoControl API");
                success = true;
                break
            },
            false => {
                attempts = attempts -1;
                let err = unsafe { Foundation::GetLastError() };
                error!("Failed to lock volume with DeviceIOControl. {} attempts remaining. Err: {:?}", attempts, err);
            }
        }
    }
    // Publish error message to front-end if volume lock does not succeed
    match success {
        true => Ok(()),
        false => Err(ImagerError::VolumeLock)
    }
}

#[cfg(target_os = "windows")]
pub fn unlock_volume(handle: Foundation::HANDLE, ) -> Result<(), ImagerError>{
    info!("Calling FSCTL_UNLOCK_VOLUME via DeviceIoControl API");
    let mut attempts = 20;
    let mut success: bool = false;
    while attempts > 0 {
        let result = unsafe { IO::DeviceIoControl(
            handle, 
            Ioctl::FSCTL_UNLOCK_VOLUME,
             None,
            None,
            None, 
            None, ) };
        match result.as_bool() {
            true => { 
                info!("Success! Unlocked volume with DeviceIoControl API");
                success = true;
                break
            },
            false => {
                attempts = attempts -1;
                let err = unsafe { Foundation::GetLastError() };
                error!("Failed to unlock volume with DeviceIOControl. {} attempts remaining. Err: {:?}", attempts, err);
            }
        }
    }
    // Publish error message to front-end if volume lock does not succeed
    match success {
        true => Ok(()),
        false => Err(ImagerError::VolumeLock)
    }
}

#[cfg(target_os = "windows")]
pub fn write_image(image_path: String, disk_path: String, device_id: String) -> Result<()> {
    let re = Regex::new(r"PHYSICALDRIVE(\d+)")?;
    let caps = re.captures(&disk_path).unwrap();
    let drivenum = caps.get(1).unwrap().as_str();
    info!("Attempting to reformat disk {}", &disk_path);


    info!("write_image called with image_path={} disk_path={} device_id={}", &image_path, &disk_path, &device_id);
    let image_file = File::open(&image_path)?;
    let bytes_total = image_file.metadata()?.len();
    let payload = WriteImageProgress {
        bytes_written: 0_u64,
        bytes_total,
        label: "Formatting disk".to_string(),
        elapsed: 0_u64,
    };
    app::TauriApp::emit(app::EVENT_IMAGE_WRITE_PROGRESS, payload);
    partition_disk_windows(&drivenum)?;
    let payload = WriteImageProgress {
        bytes_written: 0_u64,
        bytes_total,
        label: "Copying image to disk".to_string(),
        elapsed: 0_u64,
    };
    app::TauriApp::emit(app::EVENT_IMAGE_WRITE_PROGRESS, payload);

    // Reverse query for letter associated with logical disk partition
    // ref: https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-tasks--disks-and-file-systems
    // let drive_letter = get_drive_letter_windows(&device_id)?;
    // let drive_path = format!("\\\\.\\{}", &drive_letter);

    let image_file = File::open(&image_path)?;
    let drive_path = Path::new(&disk_path);
    info!("Attempting to open drive_path: {:?}", &drive_path.display());
    let lpfilename = PCSTR(drive_path.display().to_string().as_ptr());
    // get a file handle for physical disk
    info!("Attempting to open physical disk with CreateFileA API");
    let disk_handle  = unsafe { FileSystem::CreateFileA(
        lpfilename,
        // FILE_ACCESS_FLAGS
        FileSystem::FILE_GENERIC_READ | FileSystem::FILE_GENERIC_WRITE,
        // FILE_SHARE_MODE
        FileSystem::FILE_SHARE_READ,
        // SECURITY ATTRIBUTES
        None,
        // FILE_CREATION_DISPOSITION
        FileSystem::OPEN_EXISTING,
        FileSystem::FILE_ATTRIBUTE_NORMAL | FileSystem::FILE_FLAG_SEQUENTIAL_SCAN,
        None
    )? };
    info!("Success! Acquired a handle with CreateFileA API");

    // lock the volume
    lock_volume(disk_handle)?;

    // attempt to open the physical disk using a raw handle
    let outf = unsafe { File::from_raw_handle(disk_handle.0 as _) };

    // 32 MB
    let capacity = 16777216;
    let mut bytes_written = 0_u64;
    let mut image_reader = BufReader::with_capacity(capacity, image_file);
    let mut image_writer = BufWriter::with_capacity(capacity, outf);
    let now = Instant::now();
    let mut last_update = now.elapsed().as_secs();
    let update_interval = 1_u64;

    info!("Writing image {} to {}", &image_path, &disk_path);
    loop {
        let buf = image_reader.fill_buf()?;
        if buf.is_empty() {
            break;
        }
        image_writer.write_all(buf)?;
        let length = buf.len();
        bytes_written += length as u64;
        let elapsed = now.elapsed().as_secs();
        image_reader.consume(length);
        if elapsed - last_update > update_interval {
            let payload = WriteImageProgress {
                bytes_written,
                bytes_total,
                label: "Writing...".to_string(),
                elapsed,
            };
            app::TauriApp::emit(app::EVENT_IMAGE_WRITE_PROGRESS, payload);
        }
        last_update = elapsed;
    }
    info!("Finished writing {} to {}", &image_path, &disk_path);
    // unlock the volume
    unlock_volume(disk_handle)?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn write_image(image_path: String, disk: String, _deviceId: String) -> Result<()> {
    use nix::sys::socket;
    use std::{io::BufWriter, os::unix::io::FromRawFd, os::unix::prelude::RawFd};

    let image_file = File::open(&image_path)?;
    let bytes_total = image_file.metadata()?.len();
    // ensure disk is unmounted
    unmount_disk_darwin(&disk, bytes_total)?;

    let payload = WriteImageProgress {
        bytes_written: 0_u64,
        bytes_total,
        label: "Waiting for authorization".to_string(),
        elapsed: 0_u64,
    };
    app::TauriApp::emit(app::EVENT_IMAGE_WRITE_PROGRESS, payload);

    info!("Writing {} to {}", &image_path, &disk);

    let auth = authext_darwin(&disk)?;
    let external_form = auth.make_external_form()?;
    let external_form_bytes = unsafe {
        std::slice::from_raw_parts(
            external_form.bytes.as_ptr() as *const u8,
            external_form.bytes.len(),
        )
    };
    // let (sock_a, mut sock_b) = UnixStream::pair()?;
    let (sock_a, sock_b) = socket::socketpair(
        socket::AddressFamily::Unix,
        socket::SockType::Stream,
        None,
        socket::SockFlag::empty(),
    )?;

    let auth_stdout_fd = unsafe { Stdio::from_raw_fd(sock_a) };
    // let stdout_fd: RawFd = sock_a.into_raw_fd();
    // let stdoutio = unsafe { Stdio::from_raw_fd(stdout_fd) };
    let child = Command::new("/usr/libexec/authopen")
        .stdin(Stdio::piped())
        .stdout(auth_stdout_fd)
        .args(["-stdoutpipe", "-extauth", "-w", &disk])
        .spawn()?;

    // write AuthorizationExternalForm bytes to stdin, expected by `-extauth` flag
    let mut auth_stdin = child.stdin.unwrap();
    auth_stdin.write_all(external_form_bytes)?;

    // -stdoutpipe will sendmsg with SCM_RIGHTS msg type, containing a file descriptor
    let mut data = [1; 8];
    let iov = IoSliceMut::new(&mut data);
    let mut cmsg_buffer = nix::cmsg_space!([RawFd; 1]);

    let msg = socket::recvmsg::<socket::UnixAddr>(
        sock_b,
        &mut [iov],
        Some(&mut cmsg_buffer),
        socket::MsgFlags::empty(),
    )?;
    let cmsg_fd: i32 = match msg.cmsgs().next() {
        Some(socket::ControlMessageOwned::ScmRights(fds)) => fds[0],
        Some(_) => panic!("Unexpected control message"),
        None => panic!("No control message"),
    };

    let outf = unsafe { File::from_raw_fd(cmsg_fd) };

    info!("Received msg: {:?}", msg);
    // 32 MB
    let capacity = 16777216;
    let mut bytes_written = 0_u64;
    let mut image_reader = BufReader::with_capacity(capacity, image_file);
    let mut image_writer = BufWriter::with_capacity(capacity, outf);
    let now = Instant::now();
    let mut last_update = now.elapsed().as_secs();
    let update_interval = 1_u64;

    loop {
        let buf = image_reader.fill_buf()?;
        if buf.is_empty() {
            break;
        }
        image_writer.write_all(buf)?;
        let length = buf.len();
        bytes_written += length as u64;
        let elapsed = now.elapsed().as_secs();
        image_reader.consume(length);
        if elapsed - last_update > update_interval {
            let payload = WriteImageProgress {
                bytes_written,
                bytes_total,
                label: "Writing...".to_string(),
                elapsed,
            };
            app::TauriApp::emit(app::EVENT_IMAGE_WRITE_PROGRESS, payload);
        }
        last_update = elapsed;
    }
    info!("Finished writing {} to {}", &image_path, &disk);

    Ok(())
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[cfg(target_os = "macos")]
//     #[test]
//     fn test_parse_macos_dd_progress() {
//         let bytes_total = 6224347136 as u64;
//         let input = "3112173568 bytes (3112 MB, 2968 MiB) transferred 334.790s, 9296 kB/s";
//         let event = WriteImageProgress::new(input, bytes_total);

//         assert_eq!(event.bytes_total, bytes_total);
//         assert_eq!(event.bytes_written, 3112173568 as u64);
//         assert_eq!(event.percent_complete, 50);
//         assert_eq!(event.write_speed, "9296 kB/s");
//         assert_eq!(event.stdout_line, input.to_string());
//     }
// }
