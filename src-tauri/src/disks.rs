use human_bytes::human_bytes;
use serde::{Deserialize, Serialize};
use sysinfo::{DiskExt, System, SystemExt};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializableDisk {
    pub type_: String,
    pub name: String,
    pub file_system: String,
    pub mount_point: String,
    pub total_space_bytes: u64,
    pub total_space_pretty: String,
    pub available_space_bytes: u64,
    pub available_space_pretty: String,
    pub is_removable: bool,
}

pub fn get_serializable_disks() -> String {
    let mut sys = System::new();
    sys.refresh_disks_list();
    sys.refresh_disks();

    let mut disks = vec![];
    for disk in sys.disks() {
        let total_space_bytes = disk.total_space();
        let available_space_bytes = disk.available_space();

        let total_space_pretty = human_bytes(total_space_bytes as f64);
        let available_space_pretty = human_bytes(available_space_bytes as f64);

        let serializable = SerializableDisk {
            type_: format!("{:#?}", disk.type_()).into(),
            name: disk.name().to_str().unwrap_or_else(|| "Unknown").into(),
            file_system: String::from_utf8_lossy(disk.file_system()).to_string(),
            mount_point: disk.mount_point().display().to_string(),
            is_removable: disk.is_removable(),
            total_space_bytes,
            available_space_bytes,
            total_space_pretty,
            available_space_pretty,
        };
        disks.push(serializable)
    }
    serde_json::to_string(&disks).unwrap()
}
