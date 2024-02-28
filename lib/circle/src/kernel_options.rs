use crate::wrapper::*;

pub struct KernelOptions {
}

impl KernelOptions {
    pub fn get_width() -> u32 {
        unsafe {
            kernel_options_get_width()
        }
    }

    pub fn get_height() -> u32 {
        unsafe {
            kernel_options_get_height()
        }
    }

    pub fn get_log_level() -> u32 {
        unsafe {
            kernel_options_get_log_level()
        }
    }
}
