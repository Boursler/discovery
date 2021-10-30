#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 50_u16;
    loop {
        for i in 0..8 {
            leds[(i + 1) % 8].on().ok();
            delay.delay_ms(half_period);

            leds[i].off().ok();
            delay.delay_ms(half_period);
        }
    }
}
