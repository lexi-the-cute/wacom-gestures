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

pub fn get_device_model_name(device: *const WacomDevice) -> Option<String> {
    let name: String;
    unsafe {
        let name_ptr: *const i8 = wacom_bindings::libwacom_get_model_name(device);
        if name_ptr == std::ptr::null() {
            return None;
        }

        let string_of_c: CString = CString::from_raw(name_ptr.cast_mut());
        name = string_of_c.into_string().unwrap();
    }

    return Some(name);
}

pub fn get_device_layout_name(device: *const WacomDevice) -> Option<String> {
    let name: String;
    unsafe {
        let name_ptr: *const i8 = wacom_bindings::libwacom_get_layout_filename(device);
        if name_ptr == std::ptr::null() {
            return None;
        }

        let string_of_c: CString = CString::from_raw(name_ptr.cast_mut());
        name = string_of_c.into_string().unwrap();
    }

    return Some(name);
}

pub fn get_device_vendor_id(device: *const WacomDevice) -> u32 {
    let vendor_id: u32;
    unsafe {
        let vendor_id_signed: i32 = wacom_bindings::libwacom_get_vendor_id(device);
        vendor_id = u32::try_from(vendor_id_signed).unwrap();

        debug!("Signed: {0}, Unsigned: {1}", vendor_id_signed, vendor_id)
    }

    return vendor_id;
}

pub fn get_device_product_id(device: *const WacomDevice) -> u32 {
    let vendor_id: u32;
    unsafe {
        let vendor_id_signed: i32 = wacom_bindings::libwacom_get_product_id(device);
        vendor_id = u32::try_from(vendor_id_signed).unwrap();

        debug!("Signed: {0}, Unsigned: {1}", vendor_id_signed, vendor_id)
    }

    return vendor_id;
}

pub fn get_device_hardware_id(device: *const WacomDevice) -> (u32, u32) {
    return (get_device_vendor_id(device), get_device_product_id(device));
}

// TODO: Fix: It keeps alternating which input to break the UTF-8 on
// pub fn get_device_match(device: *const WacomDevice) -> Option<String> {
//     let name: String;
//     unsafe {
//         let name_ptr: *const i8 = wacom_bindings::libwacom_get_match(device);
//         let string_of_c: CString = CString::from_raw(name_ptr.cast_mut());
//         let string_of_c_result: std::result::Result<String, std::ffi::IntoStringError> = string_of_c.into_string();

//         if string_of_c_result.is_err() {
//             return None;
//         }

//         name = string_of_c_result.unwrap();
//     }

//     return Some(name);
// }

pub fn get_device_width(device: *const WacomDevice) -> i32 {
    let width: i32;
    unsafe {
        width = wacom_bindings::libwacom_get_width(device);
    }

    return width;
}

pub fn get_device_height(device: *const WacomDevice) -> i32 {
    let height: i32;
    unsafe {
        height = wacom_bindings::libwacom_get_height(device);
    }

    return height;
}

pub fn get_device_size(device: *const WacomDevice) -> (i32, i32) {
    return (get_device_width(device), get_device_height(device));
}

pub fn device_has_styli_support(device: *const WacomDevice) -> bool {
    unsafe {
        let support_int: i32 = wacom_bindings::libwacom_has_stylus(device);
        if support_int > 0 {
            return true;
        }
    }

    return false;
}

pub fn device_has_touch_support(device: *const WacomDevice) -> bool {
    unsafe {
        let support_int: i32 = wacom_bindings::libwacom_has_touch(device);
        if support_int > 0 {
            return true;
        }
    }

    return false;
}

pub fn get_device_num_rings(device: *const WacomDevice) -> i32 {
    let mut num_rings: i32 = 0;
    unsafe {
        let has_first_ring_int: i32 = wacom_bindings::libwacom_has_ring(device);
        if has_first_ring_int > 0 {
            num_rings += 1;
        }

        let has_second_ring_int: i32 = wacom_bindings::libwacom_has_ring2(device);
        if has_second_ring_int > 0 {
            num_rings += 1;
        }
    }

    return num_rings;
}

pub fn device_has_touch_switch(device: *const WacomDevice) -> bool {
    unsafe {
        let support_int: i32 = wacom_bindings::libwacom_has_touchswitch(device);
        if support_int > 0 {
            return true;
        }
    }

    return false;
}

pub fn get_device_num_strips(device: *const WacomDevice) -> i32 {
    let num_strips: i32;
    unsafe {
        num_strips = wacom_bindings::libwacom_get_num_strips(device);
    }

    return num_strips;
}

pub fn device_is_reversible(device: *const WacomDevice) -> bool {
    unsafe {
        let support_int: i32 = wacom_bindings::libwacom_is_reversible(device);
        if support_int > 0 {
            return true;
        }
    }

    return false;
}