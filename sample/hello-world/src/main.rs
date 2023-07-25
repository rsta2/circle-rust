#![no_std]
#![no_main]

use circle::circle_init;
use circle::act_led::ActLED;
use circle::screen::ScreenDevice;
use circle::wrapper::EXIT_HALT;

#[no_mangle]
pub extern "C" fn main() -> u32 {
    circle_init();

    let act_led = ActLED::new();
    act_led.blink(5, 200, 500);

    let screen = ScreenDevice::new(0, 0, 0);
    screen.write("Hello, world!\n");

    EXIT_HALT
}
