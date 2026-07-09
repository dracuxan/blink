#![no_std]
#![no_main]

use panic_halt as _;

const START_BYTE: u8 = 0x53;
const FINISH_BYTE: u8 = 0x46;
const COMMAND_HEADER: u8 = 0xAA;
const RESPONSE_HEADER: u8 = 0x55;

const TARGET_X: u8 = 76;
const TARGET_Y: u8 = 192;
const HIT_RADIUS: u16 = 0;

const SERVO_MIN: u16 = 250;
const SERVO_MAX: u16 = 500;
const SERVO_SETTLE_MS: u16 = 300;

struct Button {
    pressed: bool,
}

struct Analog {
    x: u8,
    y: u8,
    button: Button,
}

fn read_analog_axis_level(value: u16) -> u8 {
    ((value as u32 * 255) / 1023) as u8
}

fn map_to_servo(axis: u8) -> u16 {
    SERVO_MIN + (axis as u16 * (SERVO_MAX - SERVO_MIN) / 255)
}

fn checksum(bytes: &[u8]) -> u8 {
    let mut result = 0;

    for byte in bytes {
        result ^= *byte;
    }

    result
}

fn evaluate_target(x: u8, y: u8) -> (u8, u16) {
    let dx = x.abs_diff(TARGET_X) as u16;
    let dy = y.abs_diff(TARGET_Y) as u16;
    let distance = dx + dy;

    let hit = if distance <= HIT_RADIUS { 1 } else { 0 };
    let fitness = 510 - distance;

    (hit, fitness)
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let analog_x_pin = pins.a2.into_analog_input(&mut adc);
    let analog_y_pin = pins.a3.into_analog_input(&mut adc);
    let analog_button_pin = pins.a4.into_pull_up_input();

    let mut laser_pin = pins.d7.into_output();
    laser_pin.set_high();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    pins.d9.into_output();
    pins.d10.into_output();

    let tc1 = dp.TC1;

    tc1.icr1().write(|w| unsafe { w.bits(4999) });

    tc1.tccr1a()
        .write(|w| unsafe { w.wgm1().bits(0b10).com1a().bits(0b10).com1b().bits(0b10) });

    tc1.tccr1b()
        .write(|w| unsafe { w.wgm1().bits(0b11).cs1().prescale_64() });

    loop {
        // Manual joystick mode
        let mut previous_button = false;

        loop {
            let analog = Analog {
                x: read_analog_axis_level(analog_x_pin.analog_read(&mut adc)),
                y: read_analog_axis_level(analog_y_pin.analog_read(&mut adc)),
                button: Button {
                    pressed: analog_button_pin.is_low(),
                },
            };

            tc1.ocr1a()
                .write(|w| unsafe { w.bits(map_to_servo(analog.x)) });

            tc1.ocr1b()
                .write(|w| unsafe { w.bits(map_to_servo(analog.y)) });

            if analog.button.pressed && !previous_button {
                serial.write_byte(START_BYTE);
                break;
            }

            previous_button = analog.button.pressed;
            arduino_hal::delay_ms(20);
        }

        // Automatic GA mode
        loop {
            let header = serial.read_byte();

            if header == FINISH_BYTE {
                break;
            }

            if header != COMMAND_HEADER {
                continue;
            }

            let x = serial.read_byte();
            let y = serial.read_byte();
            let received_checksum = serial.read_byte();

            let expected_checksum = checksum(&[COMMAND_HEADER, x, y]);

            if received_checksum != expected_checksum {
                continue;
            }

            tc1.ocr1a().write(|w| unsafe { w.bits(map_to_servo(x)) });

            tc1.ocr1b().write(|w| unsafe { w.bits(map_to_servo(y)) });

            arduino_hal::delay_ms(SERVO_SETTLE_MS.into());

            let (hit, fitness) = evaluate_target(x, y);
            let fitness_high = (fitness >> 8) as u8;
            let fitness_low = fitness as u8;

            let response_checksum =
                checksum(&[RESPONSE_HEADER, x, y, hit, fitness_high, fitness_low]);

            serial.write_byte(RESPONSE_HEADER);
            serial.write_byte(x);
            serial.write_byte(y);
            serial.write_byte(hit);
            serial.write_byte(fitness_high);
            serial.write_byte(fitness_low);
            serial.write_byte(response_checksum);
        }

        // GA has finished. Wait for a fresh joystick press.
        while analog_button_pin.is_low() {
            arduino_hal::delay_ms(20);
        }

        while !analog_button_pin.is_low() {
            arduino_hal::delay_ms(20);
        }

        // Consume the release so it does not immediately start GA again.
        while analog_button_pin.is_low() {
            arduino_hal::delay_ms(20);
        }

        // The outer loop now returns to manual mode.
    }
}
