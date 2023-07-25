#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod wrapper {
    include!("../../wrapper/wrapper.rs");
}

pub fn circle_init() {
    unsafe {
        crate::wrapper::circle_init();
    }
}

pub mod act_led;
pub mod screen;

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_ : &PanicInfo) -> ! {
    loop {}
}
