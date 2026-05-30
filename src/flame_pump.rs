pub fn run() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut sensor = pins.a0.into_analog_input(&mut adc);

    let mut ena = pins.d9.into_output();
    let mut in1 = pins.d8.into_output();
    let mut in2 = pins.d7.into_output();

    ena.set_high();

    loop {
        let val: u16 = adc.read_blocking(&mut sensor);
        ufmt::uwriteln!(&mut serial, "{}", val).unwrap();

        if val < 125 {
            in1.set_high();
            in2.set_low();
        } else {
            in1.set_low();
            in2.set_low();
        }

        arduino_hal::delay_ms(100);
    }
}
