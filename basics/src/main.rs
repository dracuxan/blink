#![no_std]
#![no_main]
use panic_halt as _;

#[allow(dead_code)]
mod blink_led;
#[allow(dead_code)]
mod detect_laser;
#[allow(dead_code)]
mod flame_pump;

#[arduino_hal::entry]
fn main() -> ! {
    // Call whichever module you want to flash:
    blink_led::run();
}
