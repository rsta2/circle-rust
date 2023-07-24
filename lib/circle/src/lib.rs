#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod wrapper {
    include!("../../wrapper/wrapper.rs");
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_ : &PanicInfo) -> ! {
    loop {}
}
