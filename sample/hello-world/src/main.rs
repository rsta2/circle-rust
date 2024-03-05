#![no_std]
#![no_main]

use circle::circle_init;
use circle::act_led::ActLED;
use circle::kernel_options::KernelOptions;
use circle::device::Device;
use circle::screen::ScreenDevice;
use circle::serial::SerialDevice;
use circle::logger::*;
use circle::wrapper::*;

#[no_mangle]
pub extern "C" fn main() -> u32 {
    circle_init();

    let act_led = ActLED::new();
    act_led.blink(5, 200, 500);

    let screen = ScreenDevice::new(KernelOptions::get_width(), KernelOptions::get_height(), 0);

    let serial_num = KernelOptions::get_log_serial_device_num();
    let serial = SerialDevice::new(115200, 8, 1, serial_parity_t_serial_parity_none,
                                   if serial_num >= 0 { serial_num as u32 } else { 0 });

    let console: &dyn Device = if serial_num >= 0 { &serial } else { &screen };
    console.write("Hello, world!\n");

    Logger::init(console, KernelOptions::get_log_level());
    Logger::write("main", log_severity_t_log_notice, "My log message");

    EXIT_HALT
}
