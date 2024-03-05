use crate::wrapper::*;
use core::str;

pub trait Device {
    fn get_handle(&self) -> circle_handle_t;
    fn write(&self, string: &str);
}
