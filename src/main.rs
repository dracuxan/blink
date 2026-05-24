#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut pin2 = pins.d2.into_output();
    let mut pin4 = pins.d4.into_output();
    let mut pin6 = pins.d6.into_output();
    let mut pin8 = pins.d8.into_output();

    loop {
        pin2.set_high();
        arduino_hal::delay_ms(1000);
        pin2.set_low();

        pin4.set_high();
        arduino_hal::delay_ms(1000);
        pin4.set_low();

        pin6.set_high();
        arduino_hal::delay_ms(1000);
        pin6.set_low();

        pin8.set_high();
        arduino_hal::delay_ms(1000);
        pin8.set_low();
    }
}
