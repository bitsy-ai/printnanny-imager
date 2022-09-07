use serde::{Deserialize, Serialize};
use sysinfo::{DiskExt, System, SystemExt};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializableDisk {
    pub type_: String,
    pub name: String,
    pub file_system: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
}

pub fn get_serializable_disks() -> String {
    let mut sys = System::new();
    sys.refresh_disks_list();
    sys.refresh_disks();

    let mut disks = vec![];
    for disk in sys.disks() {
        let serializable = SerializableDisk {
            type_: format!("{:#?}", disk.type_()).into(),
            name: disk.name().to_str().unwrap_or_else(|| "Unknown").into(),
            file_system: String::from_utf8_lossy(disk.file_system()).to_string(),
            mount_point: disk.mount_point().display().to_string(),
            is_removable: disk.is_removable(),
            available_space: disk.available_space(),
            total_space: disk.total_space(),
        };
        disks.push(serializable)
    }
    serde_json::to_string(&disks).unwrap()
}
