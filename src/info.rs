pub mod wacom_bindings;

use std::collections::HashMap;
use std::ffi::{CString, OsString};
use std::fs::{read_dir, ReadDir};
use std::{path::PathBuf, io::Result};
use wacom_bindings::{WacomFallbackFlags, WacomDeviceDatabase, WacomError, WacomDevice, WacomFallbackFlags_WFALLBACK_GENERIC};

pub fn get_database_and_error_structure() -> (*mut WacomDeviceDatabase, *mut WacomError) {
    unsafe {
        let db: *mut WacomDeviceDatabase = wacom_bindings::libwacom_database_new();
        let error: *mut WacomError = wacom_bindings::libwacom_error_new();

        return (db, error)
    }
}

// What...
// #[allow(dead_code)]
// fn get_device_list(db: *mut WacomDeviceDatabase, error: *mut WacomError) {
//     let devices: *mut *mut wacom_bindings::WacomDevice;
//     unsafe {
//         devices = wacom_bindings::libwacom_list_devices_from_database(db, error);
//         debug!("Devices: {:#?}", **devices);
//     }
// }

pub fn get_device_list(db: *mut WacomDeviceDatabase, error: *mut WacomError) -> HashMap<String, *mut WacomDevice> {
    let inputs: PathBuf = PathBuf::from("/").join("dev").join("input");
    let path_results: Result<ReadDir> = read_dir(inputs);
    let mut devices: HashMap<String, *mut WacomDevice> = HashMap::new();

    if path_results.is_err() {
        error!("Could not find input devices...");
        return devices;
    }

    for result in path_results.unwrap() {
        let path: PathBuf = result.as_ref().unwrap().path();
        let filename: OsString = result.as_ref().unwrap().file_name();

        if !filename.to_str().unwrap().starts_with("event") {
            continue;
        }

        debug!("Name: {}", path.display());
        let possible_device: Option<*mut wacom_bindings::WacomDevice> = get_device(db, error, path.to_str().unwrap());
        if possible_device.is_some() {
            debug!("Found device {0}...", path.display());
            devices.insert(path.to_str().unwrap().to_string(), possible_device.unwrap());
        }
    }

    return devices;
}

pub fn get_device(db: *mut WacomDeviceDatabase, error: *mut WacomError, device_path: &str) -> Option<*mut WacomDevice> {
    // Attempt to find a generic device template in place of device if finding actual device fails
    let flags: WacomFallbackFlags = WacomFallbackFlags_WFALLBACK_GENERIC;
    let path: CString = CString::new(device_path).unwrap();

    let device: *mut WacomDevice;
    unsafe {
        device = wacom_bindings::libwacom_new_from_path(db, path.as_ptr(), flags, error);
    }

    if device.is_null() {
        return None;
    }

    return Some(device);
}

pub fn get_device_name(device: *const WacomDevice) -> String {
    let name: String;
    unsafe {
        let name_ptr: *const i8 = wacom_bindings::libwacom_get_name(device);
        let string_of_c: CString = CString::from_raw(name_ptr.cast_mut());

        name = string_of_c.into_string().unwrap();
    }

    return name;
}