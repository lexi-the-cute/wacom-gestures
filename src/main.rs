
#[macro_use] extern crate log;

use std::collections::HashMap;

pub mod info;

fn main() {
    pretty_env_logger::init();

    // Setup Database
    let db: *mut info::wacom_bindings::WacomDeviceDatabase;
    let error: *mut info::wacom_bindings::WacomError;
    (db, error) = info::get_database_and_error_structure();
    let devices: HashMap<String, *mut info::wacom_bindings::WacomDevice> = info::get_device_list(db, error);

    for (path, device) in devices {
        let name: String = info::get_device_name(device);

        info!("Device: {0}, Name: {1}", path, name);
    }
}