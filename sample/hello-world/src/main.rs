#![no_std]
#![no_main]

use circle::wrapper::*;

use core::ffi::c_void;
use core::ffi::c_ulong;

#[no_mangle]
pub extern "C" fn main() -> u32 {
    unsafe {
        circle_init();

        let act_led = act_led_create();
        act_led_blink(act_led, 5, 200, 500);

        let screen = screen_device_create(0, 0, 0);
        let msg = "Hello, world!\n";
        device_write(screen, msg.as_ptr() as *const c_void, msg.len() as c_ulong);
    }

    EXIT_HALT
}
