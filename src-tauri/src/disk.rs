use std::io::{prelude::*, BufReader};
use std::process::Command;
use std::process::Stdio;
use std::string::String;
use std::time::Instant;

use anyhow::{Context, Result};
use human_bytes::human_bytes;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

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
pub fn write_image(image_path: String, disk: String) -> Result<()> {
    unimplemented!("write_image is not implemented for target_os=linux")
}

#[cfg(target_os = "windows")]
pub fn write_image(image_path: String, disk: String) -> Result<()> {
    unimplemented!("write_image is not implemented for target_os=windows")
}

#[cfg(target_os = "macos")]
pub fn write_image(image_path: String, disk: String) -> Result<()> {
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
