#![no_std]
#![no_main]

use circle::circle_init;
use circle::act_led::ActLED;
use circle::kernel_options::KernelOptions;
use circle::screen::ScreenDevice;
use circle::logger::*;
use circle::wrapper::*;

#[no_mangle]
pub extern "C" fn main() -> u32 {
    circle_init();

    let act_led = ActLED::new();
    act_led.blink(5, 200, 500);

    let screen = ScreenDevice::new(KernelOptions::get_width(), KernelOptions::get_height(), 0);
    screen.write("Hello, world!\n");

    Logger::init(&screen, KernelOptions::get_log_level());
    Logger::write("main", log_severity_t_log_notice, "My log message");

    EXIT_HALT
}
