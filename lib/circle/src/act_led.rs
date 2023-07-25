use crate::wrapper::*;

pub struct ActLED {
    handle: circle_handle_t,
}

impl ActLED {
    pub fn new() -> ActLED {
        ActLED {
            handle: unsafe {
                act_led_create()
            },
        }
    }

    pub fn on(&self) {
        unsafe {
            act_led_on(self.handle);
        }
    }

    pub fn off(&self) {
        unsafe {
            act_led_off(self.handle);
        }
    }

    pub fn blink(&self, count: u32, msec_on: u32, msec_off: u32) {
        unsafe {
            act_led_blink(self.handle, count, msec_on, msec_off);
        }
    }
}
