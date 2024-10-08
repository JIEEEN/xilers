use device::device::file_sys::FileSystem;
use device::device::spec::DeviceSpec;
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DeviceManager {
    // 각 client group(device들의 모임)마다 하나씩 존재
    // spec을 가리키는 id와 fs를 가리키는 id가 동일해야 됨 (client의 고유 id)
    id_spec_map: BTreeMap<Uuid, DeviceSpec>,
    id_fs_map: BTreeMap<Uuid, FileSystem>,
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            id_spec_map: BTreeMap::new(),
            id_fs_map: BTreeMap::new(),
        }
    }

    pub fn add_device_spec(&mut self, id: Uuid, device_spec: DeviceSpec) {
        self.id_spec_map.insert(id, device_spec);
    }

    pub fn add_device_fs(&mut self, id: Uuid, file_system: FileSystem) {
        self.id_fs_map.insert(id, file_system);
    }

    pub fn get_device_spec(&self, id: Uuid) -> Option<&DeviceSpec> {
        self.id_spec_map.get(&id)
    }

    pub fn get_device_fs(&self, id: Uuid) -> Option<&FileSystem> {
        self.id_fs_map.get(&id)
    }

    pub fn delete_device_spec(&mut self, id: Uuid) -> bool {
        match self.id_spec_map.remove(&id) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn delete_device_fs(&mut self, id: Uuid) -> bool {
        match self.id_fs_map.remove(&id) {
            Some(_) => true,
            None => false,
        }
    }
}
