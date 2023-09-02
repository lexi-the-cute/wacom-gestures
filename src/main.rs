use std::ffi::CString;

mod wacom_bindings;

use wacom_bindings::{WacomFallbackFlags, WacomDeviceDatabase, WacomError, WacomDevice, WacomFallbackFlags_WFALLBACK_GENERIC};

fn main() {
    let db: *mut WacomDeviceDatabase;
    let error: *mut WacomError;
    (db, error) = get_database_and_error_structure();

    let possible_device: Option<*mut WacomDevice> = get_device(db, error, "/dev/input/event7");
    if possible_device.is_none() {
        println!("No device connected...");
        return
    }

    let device: *mut WacomDevice = possible_device.unwrap();

    println!("Device Pointer: {:#?}", device);

    get_device_list(db, error)
}

fn get_database_and_error_structure() -> (*mut WacomDeviceDatabase, *mut WacomError) {
    unsafe {
        let db: *mut WacomDeviceDatabase = wacom_bindings::libwacom_database_new();
        let error: *mut WacomError = wacom_bindings::libwacom_error_new();

        return (db, error)
    }
}

fn get_device_list(db: *mut WacomDeviceDatabase, error: *mut WacomError) {
    let devices: *mut *mut wacom_bindings::WacomDevice;
    unsafe {
        devices = wacom_bindings::libwacom_list_devices_from_database(db, error);
        println!("Devices: {:#?}", **devices);
    }
}

fn get_device(db: *mut WacomDeviceDatabase, error: *mut WacomError, device_path: &str) -> Option<*mut WacomDevice> {
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