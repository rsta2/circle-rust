use crate::device::Device;
use crate::wrapper::*;
use core::str;
use core::ffi::c_void;
use core::ffi::c_ulong;

pub struct SerialDevice {
    handle: circle_handle_t
}

impl SerialDevice {
   pub fn new(baudrate: u32, data_bits: u32, stop_bits: u32, partity: serial_parity_t, device_num: u32) -> SerialDevice {
        SerialDevice {
            handle: unsafe {
                serial_device_create(baudrate, data_bits, stop_bits, partity, device_num)
            }
        }
    }
 }

impl Device for SerialDevice {
    fn get_handle(&self) -> circle_handle_t {
        self.handle
    }

    fn write(&self, string: &str) {
        unsafe {
            device_write(self.handle, string.as_ptr() as *const c_void, string.len() as c_ulong);
        }
    }
}
