pub fn run() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut sensor = pins.a0.into_analog_input(&mut adc);

    let mut s = pins.d8.into_output();

    loop {
        let val: u16 = adc.read_blocking(&mut sensor);

        if val < 125 {
            s.set_high();
        } else {
            s.set_low();
        }

        arduino_hal::delay_ms(100);
    }
}
