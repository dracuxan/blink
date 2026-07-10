# point

Arduino Nano firmware written in Rust for a joystick-driven two-axis servo pointer.

The firmware has two operating modes:

- Manual mode reads a joystick and directly drives two servos.
- Automatic mode accepts serial commands from a host program, moves the servos, evaluates the requested point against a configured target, and sends a scored response back.

This is intended for experiments where a host-side genetic algorithm or search process needs to try `(x, y)` positions and receive a fitness score from the board.

## Hardware

- Arduino Nano / ATmega328P
- Two servos:
  - X axis on D9 / Timer1 OCR1A
  - Y axis on D10 / Timer1 OCR1B
- Joystick:
  - X axis on A2
  - Y axis on A3
  - Button on A4, using the internal pull-up
- Laser / pointer enable on D7

Use a separate servo power supply if the servos draw more current than the Nano can safely provide. Tie the external supply ground to the Nano ground.

## Firmware Behavior

On boot, the board enters manual mode:

1. A2 and A3 are read as 10-bit analog values.
2. Each axis is scaled to `0..=255`.
3. The scaled values are mapped to the configured servo pulse range.
4. Pressing the joystick button sends `0x53` (`START_BYTE`) over serial and switches to automatic mode.

In automatic mode, the board waits for serial command packets from the host. Each valid command moves both servos, waits for the configured settle time, evaluates the point, and writes a response packet.

Sending `0x46` (`FINISH_BYTE`) exits automatic mode. The firmware then waits for a fresh joystick button press before returning to normal manual operation.

## Serial Protocol

Serial settings:

- Baud rate: `57600`
- Command header: `0xAA`
- Response header: `0x55`
- Start byte: `0x53`
- Finish byte: `0x46`
- Checksum: XOR of all packet bytes except the checksum byte itself

Host command packet:

```text
0xAA x y checksum
```

Response packet:

```text
0x55 x y hit fitness_high fitness_low checksum
```

Fields:

- `x`, `y`: requested point, each in `0..=255`
- `hit`: `1` when the point is within `HIT_RADIUS`, otherwise `0`
- `fitness`: `510 - (abs(x - TARGET_X) + abs(y - TARGET_Y))`, encoded big-endian

The current target and motion constants are defined in `src/main.rs`:

```rust
const TARGET_X: u8 = 76;
const TARGET_Y: u8 = 192;
const HIT_RADIUS: u16 = 0;

const SERVO_MIN: u16 = 250;
const SERVO_MAX: u16 = 500;
const SERVO_SETTLE_MS: u16 = 300;
```

Adjust these values for your physical setup before running experiments.

## Requirements

- Rust nightly from `rust-toolchain.toml`
- `rust-src` component
- AVR toolchain:
  - `avr-gcc`
  - `avr-libc`
  - `avrdude`
- [`ravedude`](https://crates.io/crates/ravedude)

The project uses [`avr-hal`](https://github.com/Rahix/avr-hal) with the `arduino-nano` feature enabled.

## Build and Flash

From this directory:

```bash
cargo build
```

To build, flash, and open the serial console:

```bash
cargo run
```

The Cargo configuration targets `avr-none`, sets `target-cpu=atmega328p`, builds `core`, and uses `ravedude` as the AVR runner. `Ravedude.toml` configures the board as `nano` and opens the serial console at `57600` baud after flashing.

## Project Layout

```text
.
|-- Cargo.toml
|-- Ravedude.toml
|-- rust-toolchain.toml
`-- src
    `-- main.rs
```

## Development Notes

- The firmware is `#![no_std]` and `#![no_main]`.
- Timer1 is configured directly for servo PWM on D9 and D10.
- The laser output is currently set high on startup.
- Invalid command headers and packets with bad checksums are ignored.
- `panic-halt` is used for panic behavior on the microcontroller.
