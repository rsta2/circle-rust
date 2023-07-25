use crate::wrapper::*;
use core::str;
use core::ffi::c_void;
use core::ffi::c_ulong;

pub struct ScreenDevice {
    handle: circle_handle_t,
}

impl ScreenDevice {
    pub fn new(width: u32, height: u32, display_num: u32) -> ScreenDevice {
        ScreenDevice {
            handle: unsafe {
                screen_device_create(width, height, display_num)
            },
        }
    }

    pub fn write(&self, string: &str) {
        unsafe {
            device_write(self.handle, string.as_ptr() as *const c_void, string.len() as c_ulong);
        }
    }
}
