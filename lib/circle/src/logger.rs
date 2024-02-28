use crate::wrapper::*;
use crate::screen::ScreenDevice;
use crate::util::str_as_cstr_to_buf;
use core::ffi::c_char;

pub use crate::wrapper::log_severity_t;

pub struct Logger {

}

impl Logger {
    const MAX_SOURCE_LEN: usize = 50;
    const MAX_MESSAGE_LEN: usize = 200;

    pub fn init(target: &ScreenDevice, log_level: log_severity_t) {
        unsafe {
            logger_create (target.get_handle(), log_level);
        }
    }

    pub fn write(source: &str, severity: log_severity_t, message: &str) {
        let mut src: [c_char; Self::MAX_SOURCE_LEN] = [0; Self::MAX_SOURCE_LEN];
        str_as_cstr_to_buf(source, &mut src);

        let mut msg: [c_char; Self::MAX_MESSAGE_LEN] = [0; Self::MAX_MESSAGE_LEN];
        str_as_cstr_to_buf(message, &mut msg);

        unsafe {
            logger_write (src.as_ptr(), severity, msg.as_ptr());
        }
    }
}
